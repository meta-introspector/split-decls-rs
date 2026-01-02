mkmod!{raw_dylib, { 
                getname!(raw_dylib);
                getsrc!(raw_dylib);
                getpath!(raw_dylib);
                get_deps!(raw_dylib);
                get_crates!(raw_dylib);
                mkinclude!(raw_dylib);
                 
            }}
mkuse!{use std :: collections :: BTreeSet ;}
mkuse!{use std :: ffi :: OsString ;}
mkuse!{use std :: fs :: { File , OpenOptions , read } ;}
mkuse!{use std :: io :: { BufReader , BufWriter , Write } ;}
mkuse!{use std :: ops :: { ControlFlow , Deref } ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use std :: process :: { Output , Stdio } ;}
mkuse!{use std :: { env , fmt , fs , io , mem , str } ;}
mkuse!{use cc :: windows_registry ;}
mkuse!{use itertools :: Itertools ;}
mkuse!{use regex :: Regex ;}
mkuse!{use rustc_arena :: TypedArena ;}
mkuse!{use rustc_ast :: CRATE_NODE_ID ;}
mkuse!{use rustc_attr_parsing :: { ShouldEmit , eval_config_entry } ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexSet ;}
mkuse!{use rustc_data_structures :: memmap :: Mmap ;}
mkuse!{use rustc_data_structures :: temp_dir :: MaybeTempDir ;}
mkuse!{use rustc_errors :: { DiagCtxtHandle , LintDiagnostic } ;}
mkuse!{use rustc_fs_util :: { TempDirBuilder , fix_windows_verbatim_for_gcc , try_canonicalize } ;}
mkuse!{use rustc_hir :: attrs :: NativeLibKind ;}
mkuse!{use rustc_hir :: def_id :: { CrateNum , LOCAL_CRATE } ;}
mkuse!{use rustc_macros :: LintDiagnostic ;}
mkuse!{use rustc_metadata :: fs :: { METADATA_FILENAME , copy_to_stdout , emit_wrapper_file } ;}
mkuse!{use rustc_metadata :: { EncodedMetadata , NativeLibSearchFallback , find_native_static_library , walk_native_lib_search_dirs , } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: lint :: lint_level ;}
mkuse!{use rustc_middle :: middle :: debugger_visualizer :: DebuggerVisualizerFile ;}
mkuse!{use rustc_middle :: middle :: dependency_format :: Linkage ;}
mkuse!{use rustc_middle :: middle :: exported_symbols :: SymbolExportKind ;}
mkuse!{use rustc_session :: config :: { self , CFGuard , CrateType , DebugInfo , LinkerFeaturesCli , OutFileName , OutputFilenames , OutputType , PrintKind , SplitDwarfKind , Strip , } ;}
mkuse!{use rustc_session :: lint :: builtin :: LINKER_MESSAGES ;}
mkuse!{use rustc_session :: output :: { check_file_is_writeable , invalid_output_for_target , out_filename } ;}
mkuse!{use rustc_session :: search_paths :: PathKind ;}
mkuse!{# [doc = " For all the linkers we support, and information they might"] # [doc = " need out of the shared crate context before we get rid of it."] use rustc_session :: { Session , filesearch } ;}
mkuse!{use rustc_span :: Symbol ;}
mkuse!{use rustc_target :: spec :: crt_objects :: CrtObjects ;}
mkuse!{use rustc_target :: spec :: { BinaryFormat , Cc , LinkOutputKind , LinkSelfContainedComponents , LinkSelfContainedDefault , LinkerFeatures , LinkerFlavor , LinkerFlavorCli , Lld , PanicStrategy , RelocModel , RelroLevel , SanitizerSet , SplitDebuginfo , } ;}
mkuse!{use tracing :: { debug , info , warn } ;}
mkuse!{use super :: archive :: { ArchiveBuilder , ArchiveBuilderBuilder } ;}
mkuse!{use super :: command :: Command ;}
mkuse!{use super :: linker :: { self , Linker } ;}
mkuse!{use super :: metadata :: { MetadataPosition , create_wrapper_file } ;}
mkuse!{use super :: rpath :: { self , RPathConfig } ;}
mkuse!{use super :: { apple , versioned_llvm_target } ;}
mkuse!{use crate :: base :: needs_allocator_shim_for_linking ;}
mkuse!{use crate :: { CodegenResults , CompiledModule , CrateInfo , NativeLib , errors , looks_like_rust_object_file , } ;}

macro_rules! ensure_removed_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ensure_removed in module {}", module_path!());
    };
}

mkfn!{
    ensure_removed_introspect!();
    pub fn ensure_removed (dcx : DiagCtxtHandle < '_ > , path : & Path) { if let Err (e) = fs :: remove_file (path) { if e . kind () != io :: ErrorKind :: NotFound { dcx . err (format ! ("failed to remove {}: {}" , path . display () , e)) ; } } }
}

macro_rules! link_binary_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function link_binary in module {}", module_path!());
    };
}

mkfn!{
    link_binary_introspect!();
    # [doc = " Performs the linkage portion of the compilation phase. This will generate all"] # [doc = " of the requested outputs for this compilation session."] pub fn link_binary (sess : & Session , archive_builder_builder : & dyn ArchiveBuilderBuilder , codegen_results : CodegenResults , metadata : EncodedMetadata , outputs : & OutputFilenames ,) { let _timer = sess . timer ("link_binary") ; let output_metadata = sess . opts . output_types . contains_key (& OutputType :: Metadata) ; let mut tempfiles_for_stdout_output : Vec < PathBuf > = Vec :: new () ; for & crate_type in & codegen_results . crate_info . crate_types { if (sess . opts . unstable_opts . no_codegen || ! sess . opts . output_types . should_codegen ()) && ! output_metadata && crate_type == CrateType :: Executable { continue ; } if invalid_output_for_target (sess , crate_type) { bug ! ("invalid output type `{:?}` for target `{}`" , crate_type , sess . opts . target_triple) ; } sess . time ("link_binary_check_files_are_writeable" , | | { for obj in codegen_results . modules . iter () . filter_map (| m | m . object . as_ref ()) { check_file_is_writeable (obj , sess) ; } }) ; if outputs . outputs . should_link () { let tmpdir = TempDirBuilder :: new () . prefix ("rustc") . tempdir () . unwrap_or_else (| error | sess . dcx () . emit_fatal (errors :: CreateTempDir { error })) ; let path = MaybeTempDir :: new (tmpdir , sess . opts . cg . save_temps) ; let output = out_filename (sess , crate_type , outputs , codegen_results . crate_info . local_crate_name ,) ; let crate_name = format ! ("{}" , codegen_results . crate_info . local_crate_name) ; let out_filename = output . file_for_writing (outputs , OutputType :: Exe , & crate_name , sess . invocation_temp . as_deref () ,) ; match crate_type { CrateType :: Rlib => { let _timer = sess . timer ("link_rlib") ; info ! ("preparing rlib to {:?}" , out_filename) ; link_rlib (sess , archive_builder_builder , & codegen_results , & metadata , RlibFlavor :: Normal , & path ,) . build (& out_filename) ; } CrateType :: Staticlib => { link_staticlib (sess , archive_builder_builder , & codegen_results , & metadata , & out_filename , & path ,) ; } _ => { link_natively (sess , archive_builder_builder , crate_type , & out_filename , & codegen_results , & metadata , path . as_ref () ,) ; } } if sess . opts . json_artifact_notifications { sess . dcx () . emit_artifact_notification (& out_filename , "link") ; } if sess . prof . enabled () && let Some (artifact_name) = out_filename . file_name () { let file_size = std :: fs :: metadata (& out_filename) . map (| m | m . len ()) . unwrap_or (0) ; sess . prof . artifact_size ("linked_artifact" , artifact_name . to_string_lossy () , file_size ,) ; } if sess . target . binary_format == BinaryFormat :: Elf { if let Err (err) = warn_if_linked_with_gold (sess , & out_filename) { info ! (? err , "Error while checking if gold was the linker") ; } } if output . is_stdout () { if output . is_tty () { sess . dcx () . emit_err (errors :: BinaryOutputToTty { shorthand : OutputType :: Exe . shorthand () , }) ; } else if let Err (e) = copy_to_stdout (& out_filename) { sess . dcx () . emit_err (errors :: CopyPath :: new (& out_filename , output . as_path () , e)) ; } tempfiles_for_stdout_output . push (out_filename) ; } } } sess . time ("link_binary_remove_temps" , | | { if sess . opts . cg . save_temps { return ; } let maybe_remove_temps_from_module = | preserve_objects : bool , preserve_dwarf_objects : bool , module : & CompiledModule | { if ! preserve_objects && let Some (ref obj) = module . object { ensure_removed (sess . dcx () , obj) ; } if ! preserve_dwarf_objects && let Some (ref dwo_obj) = module . dwarf_object { ensure_removed (sess . dcx () , dwo_obj) ; } } ; let remove_temps_from_module = | module : & CompiledModule | maybe_remove_temps_from_module (false , false , module) ; if let Some (ref allocator_module) = codegen_results . allocator_module { remove_temps_from_module (allocator_module) ; } for temp in tempfiles_for_stdout_output { ensure_removed (sess . dcx () , & temp) ; } if ! sess . opts . output_types . should_link () { return ; } let (preserve_objects , preserve_dwarf_objects) = preserve_objects_for_their_debuginfo (sess) ; debug ! (? preserve_objects , ? preserve_dwarf_objects) ; for module in & codegen_results . modules { maybe_remove_temps_from_module (preserve_objects , preserve_dwarf_objects , module) ; } }) ; }
}

macro_rules! each_linked_rlib_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function each_linked_rlib in module {}", module_path!());
    };
}

mkfn!{
    each_linked_rlib_introspect!();
    pub fn each_linked_rlib (info : & CrateInfo , crate_type : Option < CrateType > , f : & mut dyn FnMut (CrateNum , & Path) ,) -> Result < () , errors :: LinkRlibError > { let fmts = if let Some (crate_type) = crate_type { let Some (fmts) = info . dependency_formats . get (& crate_type) else { return Err (errors :: LinkRlibError :: MissingFormat) ; } ; fmts } else { let mut dep_formats = info . dependency_formats . iter () ; let (ty1 , list1) = dep_formats . next () . ok_or (errors :: LinkRlibError :: MissingFormat) ? ; if let Some ((ty2 , list2)) = dep_formats . find (| (_ , list2) | list1 != * list2) { return Err (errors :: LinkRlibError :: IncompatibleDependencyFormats { ty1 : format ! ("{ty1:?}") , ty2 : format ! ("{ty2:?}") , list1 : format ! ("{list1:?}") , list2 : format ! ("{list2:?}") , }) ; } list1 } ; let used_dep_crates = info . used_crates . iter () ; for & cnum in used_dep_crates { match fmts . get (cnum) { Some (& Linkage :: NotLinked | & Linkage :: Dynamic | & Linkage :: IncludedFromDylib) => continue , Some (_) => { } None => return Err (errors :: LinkRlibError :: MissingFormat) , } let crate_name = info . crate_name [& cnum] ; let used_crate_source = & info . used_crate_source [& cnum] ; if let Some ((path , _)) = & used_crate_source . rlib { f (cnum , path) ; } else if used_crate_source . rmeta . is_some () { return Err (errors :: LinkRlibError :: OnlyRmetaFound { crate_name }) ; } else { return Err (errors :: LinkRlibError :: NotFound { crate_name }) ; } } Ok (()) }
}

macro_rules! link_rlib_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function link_rlib in module {}", module_path!());
    };
}

mkfn!{
    link_rlib_introspect!();
    # [doc = " Create an 'rlib'."] # [doc = ""] # [doc = " An rlib in its current incarnation is essentially a renamed .a file (with \"dummy\" object files)."] # [doc = " The rlib primarily contains the object file of the crate, but it also some of the object files"] # [doc = " from native libraries."] fn link_rlib < 'a > (sess : & 'a Session , archive_builder_builder : & dyn ArchiveBuilderBuilder , codegen_results : & CodegenResults , metadata : & EncodedMetadata , flavor : RlibFlavor , tmpdir : & MaybeTempDir ,) -> Box < dyn ArchiveBuilder + 'a > { let mut ab = archive_builder_builder . new_archive_builder (sess) ; let trailing_metadata = match flavor { RlibFlavor :: Normal => { let (metadata , metadata_position) = create_wrapper_file (sess , ".rmeta" . to_string () , metadata . stub_or_full ()) ; let metadata = emit_wrapper_file (sess , & metadata , tmpdir . as_ref () , METADATA_FILENAME) ; match metadata_position { MetadataPosition :: First => { ab . add_file (& metadata) ; None } MetadataPosition :: Last => Some (metadata) , } } RlibFlavor :: StaticlibBase => None , } ; for m in & codegen_results . modules { if let Some (obj) = m . object . as_ref () { ab . add_file (obj) ; } if let Some (dwarf_obj) = m . dwarf_object . as_ref () { ab . add_file (dwarf_obj) ; } } match flavor { RlibFlavor :: Normal => { } RlibFlavor :: StaticlibBase => { let obj = codegen_results . allocator_module . as_ref () . and_then (| m | m . object . as_ref ()) ; if let Some (obj) = obj { ab . add_file (obj) ; } } } let mut packed_bundled_libs = Vec :: new () ; for lib in codegen_results . crate_info . used_libraries . iter () { let NativeLibKind :: Static { bundle : None | Some (true) , .. } = lib . kind else { continue ; } ; if flavor == RlibFlavor :: Normal && let Some (filename) = lib . filename { let path = find_native_static_library (filename . as_str () , true , sess) ; let src = read (path) . unwrap_or_else (| e | sess . dcx () . emit_fatal (errors :: ReadFileError { message : e })) ; let (data , _) = create_wrapper_file (sess , ".bundled_lib" . to_string () , & src) ; let wrapper_file = emit_wrapper_file (sess , & data , tmpdir . as_ref () , filename . as_str ()) ; packed_bundled_libs . push (wrapper_file) ; } else { let path = find_native_static_library (lib . name . as_str () , lib . verbatim , sess) ; ab . add_archive (& path , Box :: new (| _ | false)) . unwrap_or_else (| error | { sess . dcx () . emit_fatal (errors :: AddNativeLibrary { library_path : path , error }) }) ; } } if sess . target . is_like_windows { for output_path in raw_dylib :: create_raw_dylib_dll_import_libs (sess , archive_builder_builder , codegen_results . crate_info . used_libraries . iter () , tmpdir . as_ref () , true ,) { ab . add_archive (& output_path , Box :: new (| _ | false)) . unwrap_or_else (| error | { sess . dcx () . emit_fatal (errors :: AddNativeLibrary { library_path : output_path , error }) ; }) ; } } if let Some (trailing_metadata) = trailing_metadata { ab . add_file (& trailing_metadata) ; } for lib in packed_bundled_libs { ab . add_file (& lib) } ab }
}

macro_rules! link_staticlib_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function link_staticlib in module {}", module_path!());
    };
}

mkfn!{
    link_staticlib_introspect!();
    # [doc = " Create a static archive."] # [doc = ""] # [doc = " This is essentially the same thing as an rlib, but it also involves adding all of the upstream"] # [doc = " crates' objects into the archive. This will slurp in all of the native libraries of upstream"] # [doc = " dependencies as well."] # [doc = ""] # [doc = " Additionally, there's no way for us to link dynamic libraries, so we warn about all dynamic"] # [doc = " library dependencies that they're not linked in."] # [doc = ""] # [doc = " There's no need to include metadata in a static archive, so ensure to not link in the metadata"] # [doc = " object file (and also don't prepare the archive with a metadata file)."] fn link_staticlib (sess : & Session , archive_builder_builder : & dyn ArchiveBuilderBuilder , codegen_results : & CodegenResults , metadata : & EncodedMetadata , out_filename : & Path , tempdir : & MaybeTempDir ,) { info ! ("preparing staticlib to {:?}" , out_filename) ; let mut ab = link_rlib (sess , archive_builder_builder , codegen_results , metadata , RlibFlavor :: StaticlibBase , tempdir ,) ; let mut all_native_libs = vec ! [] ; let res = each_linked_rlib (& codegen_results . crate_info , Some (CrateType :: Staticlib) , & mut | cnum , path | { let lto = are_upstream_rust_objects_already_included (sess) && ! ignored_for_lto (sess , & codegen_results . crate_info , cnum) ; let native_libs = codegen_results . crate_info . native_libraries [& cnum] . iter () ; let relevant = native_libs . clone () . filter (| lib | relevant_lib (sess , lib)) ; let relevant_libs : FxIndexSet < _ > = relevant . filter_map (| lib | lib . filename) . collect () ; let bundled_libs : FxIndexSet < _ > = native_libs . filter_map (| lib | lib . filename) . collect () ; ab . add_archive (path , Box :: new (move | fname : & str | { if fname == METADATA_FILENAME { return true ; } if lto && looks_like_rust_object_file (fname) { return true ; } if bundled_libs . contains (& Symbol :: intern (fname)) { return true ; } false }) ,) . unwrap () ; archive_builder_builder . extract_bundled_libs (path , tempdir . as_ref () , & relevant_libs) . unwrap_or_else (| e | sess . dcx () . emit_fatal (e)) ; for filename in relevant_libs . iter () { let joined = tempdir . as_ref () . join (filename . as_str ()) ; let path = joined . as_path () ; ab . add_archive (path , Box :: new (| _ | false)) . unwrap () ; } all_native_libs . extend (codegen_results . crate_info . native_libraries [& cnum] . iter () . cloned ()) ; } ,) ; if let Err (e) = res { sess . dcx () . emit_fatal (e) ; } ab . build (out_filename) ; let crates = codegen_results . crate_info . used_crates . iter () ; let fmts = codegen_results . crate_info . dependency_formats . get (& CrateType :: Staticlib) . expect ("no dependency formats for staticlib") ; let mut all_rust_dylibs = vec ! [] ; for & cnum in crates { let Some (Linkage :: Dynamic) = fmts . get (cnum) else { continue ; } ; let crate_name = codegen_results . crate_info . crate_name [& cnum] ; let used_crate_source = & codegen_results . crate_info . used_crate_source [& cnum] ; if let Some ((path , _)) = & used_crate_source . dylib { all_rust_dylibs . push (& * * path) ; } else if used_crate_source . rmeta . is_some () { sess . dcx () . emit_fatal (errors :: LinkRlibError :: OnlyRmetaFound { crate_name }) ; } else { sess . dcx () . emit_fatal (errors :: LinkRlibError :: NotFound { crate_name }) ; } } all_native_libs . extend_from_slice (& codegen_results . crate_info . used_libraries) ; for print in & sess . opts . prints { if print . kind == PrintKind :: NativeStaticLibs { print_native_static_libs (sess , & print . out , & all_native_libs , & all_rust_dylibs) ; } } }
}

macro_rules! link_dwarf_object_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function link_dwarf_object in module {}", module_path!());
    };
}

mkfn!{
    link_dwarf_object_introspect!();
    # [doc = " Use `thorin` (rust implementation of a dwarf packaging utility) to link DWARF objects into a"] # [doc = " DWARF package."] fn link_dwarf_object (sess : & Session , cg_results : & CodegenResults , executable_out_filename : & Path) { let mut dwp_out_filename = executable_out_filename . to_path_buf () . into_os_string () ; dwp_out_filename . push (".dwp") ; debug ! (? dwp_out_filename , ? executable_out_filename) ; # [derive (Default)] struct ThorinSession < Relocations > { arena_data : TypedArena < Vec < u8 > > , arena_mmap : TypedArena < Mmap > , arena_relocations : TypedArena < Relocations > , } impl < Relocations > ThorinSession < Relocations > { fn alloc_mmap (& self , data : Mmap) -> & Mmap { & * self . arena_mmap . alloc (data) } } impl < Relocations > thorin :: Session < Relocations > for ThorinSession < Relocations > { fn alloc_data (& self , data : Vec < u8 >) -> & [u8] { & * self . arena_data . alloc (data) } fn alloc_relocation (& self , data : Relocations) -> & Relocations { & * self . arena_relocations . alloc (data) } fn read_input (& self , path : & Path) -> std :: io :: Result < & [u8] > { let file = File :: open (& path) ? ; let mmap = (unsafe { Mmap :: map (file) }) ? ; Ok (self . alloc_mmap (mmap)) } } match sess . time ("run_thorin" , | | -> Result < () , thorin :: Error > { let thorin_sess = ThorinSession :: default () ; let mut package = thorin :: DwarfPackage :: new (& thorin_sess) ; match sess . opts . unstable_opts . split_dwarf_kind { SplitDwarfKind :: Single => { for input_obj in cg_results . modules . iter () . filter_map (| m | m . object . as_ref ()) { package . add_input_object (input_obj) ? ; } } SplitDwarfKind :: Split => { for input_obj in cg_results . modules . iter () . filter_map (| m | m . dwarf_object . as_ref ()) { package . add_input_object (input_obj) ? ; } } } let input_rlibs = cg_results . crate_info . used_crate_source . items () . filter_map (| (_ , csource) | csource . rlib . as_ref ()) . map (| (path , _) | path) . into_sorted_stable_ord () ; for input_rlib in input_rlibs { debug ! (? input_rlib) ; package . add_input_object (input_rlib) ? ; } package . add_executable (executable_out_filename , thorin :: MissingReferencedObjectBehaviour :: Skip ,) ? ; let output_stream = BufWriter :: new (OpenOptions :: new () . read (true) . write (true) . create (true) . truncate (true) . open (dwp_out_filename) ? ,) ; let mut output_stream = thorin :: object :: write :: StreamingBuffer :: new (output_stream) ; package . finish () ? . emit (& mut output_stream) ? ; output_stream . result () ? ; output_stream . into_inner () . flush () ? ; Ok (()) }) { Ok (()) => { } Err (e) => sess . dcx () . emit_fatal (errors :: ThorinErrorWrapper (e)) , } }
}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (codegen_ssa_linker_output)] # [doc = " Translating this is kind of useless. We don't pass translation flags to the linker, so we'd just"] # [doc = " end up with inconsistent languages within the same diagnostic."] struct LinkerOutput { inner : String , }}}

macro_rules! link_natively_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function link_natively in module {}", module_path!());
    };
}

mkfn!{
    link_natively_introspect!();
    # [doc = " Create a dynamic library or executable."] # [doc = ""] # [doc = " This will invoke the system linker/cc to create the resulting file. This links to all upstream"] # [doc = " files as well."] fn link_natively (sess : & Session , archive_builder_builder : & dyn ArchiveBuilderBuilder , crate_type : CrateType , out_filename : & Path , codegen_results : & CodegenResults , metadata : & EncodedMetadata , tmpdir : & Path ,) { info ! ("preparing {:?} to {:?}" , crate_type , out_filename) ; let (linker_path , flavor) = linker_and_flavor (sess) ; let self_contained_components = self_contained_components (sess , crate_type , & linker_path) ; let should_archive = crate_type != CrateType :: Executable && sess . target . is_like_aix ; let archive_member = should_archive . then (| | tmpdir . join (out_filename . file_name () . unwrap ()) . with_extension ("so")) ; let temp_filename = archive_member . as_deref () . unwrap_or (out_filename) ; let mut cmd = linker_with_args (& linker_path , flavor , sess , archive_builder_builder , crate_type , tmpdir , temp_filename , codegen_results , metadata , self_contained_components ,) ; linker :: disable_localization (& mut cmd) ; for (k , v) in sess . target . link_env . as_ref () { cmd . env (k . as_ref () , v . as_ref ()) ; } for k in sess . target . link_env_remove . as_ref () { cmd . env_remove (k . as_ref ()) ; } for print in & sess . opts . prints { if print . kind == PrintKind :: LinkArgs { let content = format ! ("{cmd:?}\n") ; print . out . overwrite (& content , sess) ; } } sess . dcx () . abort_if_errors () ; info ! ("{cmd:?}") ; let unknown_arg_regex = Regex :: new (r"(unknown|unrecognized) (command line )?(option|argument)") . unwrap () ; let mut prog ; loop { prog = sess . time ("run_linker" , | | exec_linker (sess , & cmd , out_filename , flavor , tmpdir)) ; let Ok (ref output) = prog else { break ; } ; if output . status . success () { break ; } let mut out = output . stderr . clone () ; out . extend (& output . stdout) ; let out = String :: from_utf8_lossy (& out) ; if matches ! (flavor , LinkerFlavor :: Gnu (Cc :: Yes , _)) && unknown_arg_regex . is_match (& out) && out . contains ("-no-pie") && cmd . get_args () . iter () . any (| e | e == "-no-pie") { info ! ("linker output: {:?}" , out) ; warn ! ("Linker does not support -no-pie command line option. Retrying without.") ; for arg in cmd . take_args () { if arg != "-no-pie" { cmd . arg (arg) ; } } info ! ("{cmd:?}") ; continue ; } if matches ! (flavor , LinkerFlavor :: Gnu (Cc :: Yes , Lld :: Yes)) && unknown_arg_regex . is_match (& out) && out . contains ("-fuse-ld=lld") && cmd . get_args () . iter () . any (| e | e . to_string_lossy () == "-fuse-ld=lld") { info ! ("linker output: {:?}" , out) ; info ! ("The linker driver does not support `-fuse-ld=lld`. Retrying without it.") ; for arg in cmd . take_args () { if arg . to_string_lossy () != "-fuse-ld=lld" { cmd . arg (arg) ; } } info ! ("{cmd:?}") ; continue ; } if matches ! (flavor , LinkerFlavor :: Gnu (Cc :: Yes , _)) && unknown_arg_regex . is_match (& out) && (out . contains ("-static-pie") || out . contains ("--no-dynamic-linker")) && cmd . get_args () . iter () . any (| e | e == "-static-pie") { info ! ("linker output: {:?}" , out) ; warn ! ("Linker does not support -static-pie command line option. Retrying with -static instead.") ; let self_contained_crt_objects = self_contained_components . is_crt_objects_enabled () ; let opts = & sess . target ; let pre_objects = if self_contained_crt_objects { & opts . pre_link_objects_self_contained } else { & opts . pre_link_objects } ; let post_objects = if self_contained_crt_objects { & opts . post_link_objects_self_contained } else { & opts . post_link_objects } ; let get_objects = | objects : & CrtObjects , kind | { objects . get (& kind) . iter () . copied () . flatten () . map (| obj | { get_object_file_path (sess , obj , self_contained_crt_objects) . into_os_string () }) . collect :: < Vec < _ > > () } ; let pre_objects_static_pie = get_objects (pre_objects , LinkOutputKind :: StaticPicExe) ; let post_objects_static_pie = get_objects (post_objects , LinkOutputKind :: StaticPicExe) ; let mut pre_objects_static = get_objects (pre_objects , LinkOutputKind :: StaticNoPicExe) ; let mut post_objects_static = get_objects (post_objects , LinkOutputKind :: StaticNoPicExe) ; assert ! (pre_objects_static . is_empty () || ! pre_objects_static_pie . is_empty ()) ; assert ! (post_objects_static . is_empty () || ! post_objects_static_pie . is_empty ()) ; for arg in cmd . take_args () { if arg == "-static-pie" { cmd . arg ("-static") ; } else if pre_objects_static_pie . contains (& arg) { cmd . args (mem :: take (& mut pre_objects_static)) ; } else if post_objects_static_pie . contains (& arg) { cmd . args (mem :: take (& mut post_objects_static)) ; } else { cmd . arg (arg) ; } } info ! ("{cmd:?}") ; continue ; } break ; } match prog { Ok (prog) => { let is_msvc_link_exe = sess . target . is_like_msvc && flavor == LinkerFlavor :: Msvc (Lld :: No) && linker_path . to_str () == Some ("link.exe") ; if ! prog . status . success () { let mut output = prog . stderr . clone () ; output . extend_from_slice (& prog . stdout) ; let escaped_output = escape_linker_output (& output , flavor) ; let err = errors :: LinkingFailed { linker_path : & linker_path , exit_status : prog . status , command : cmd , escaped_output , verbose : sess . opts . verbose , sysroot_dir : sess . opts . sysroot . path () . to_owned () , } ; sess . dcx () . emit_err (err) ; if let Some (code) = prog . status . code () { if is_msvc_link_exe && (code < 1000 || code > 9999) { let is_vs_installed = windows_registry :: find_vs_version () . is_ok () ; let has_linker = windows_registry :: find_tool (& sess . target . arch , "link.exe") . is_some () ; sess . dcx () . emit_note (errors :: LinkExeUnexpectedError) ; const STATUS_STACK_BUFFER_OVERRUN : i32 = 0xc0000409u32 as _ ; if code == STATUS_STACK_BUFFER_OVERRUN { sess . dcx () . emit_note (errors :: LinkExeStatusStackBufferOverrun) ; } if is_vs_installed && has_linker { sess . dcx () . emit_note (errors :: RepairVSBuildTools) ; sess . dcx () . emit_note (errors :: MissingCppBuildToolComponent) ; } else if is_vs_installed { sess . dcx () . emit_note (errors :: SelectCppBuildToolWorkload) ; } else { sess . dcx () . emit_note (errors :: VisualStudioNotInstalled) ; } } } sess . dcx () . abort_if_errors () ; } let stderr = escape_string (& prog . stderr) ; let mut stdout = escape_string (& prog . stdout) ; info ! ("linker stderr:\n{}" , & stderr) ; info ! ("linker stdout:\n{}" , & stdout) ; if is_msvc_link_exe { if let Ok (str) = str :: from_utf8 (& prog . stdout) { let mut output = String :: with_capacity (str . len ()) ; for line in stdout . lines () { if line . starts_with ("   Creating library") || line . starts_with ("Generating code") || line . starts_with ("Finished generating code") { continue ; } output += line ; output += "\r\n" } stdout = escape_string (output . trim () . as_bytes ()) } } let level = codegen_results . crate_info . lint_levels . linker_messages ; let lint = | msg | { lint_level (sess , LINKER_MESSAGES , level , None , | diag | { LinkerOutput { inner : msg } . decorate_lint (diag) }) } ; if ! prog . stderr . is_empty () { let stderr = stderr . strip_prefix ("warning: ") . unwrap_or (& stderr) . replace (": warning: " , ": ") ; lint (format ! ("linker stderr: {stderr}")) ; } if ! stdout . is_empty () { lint (format ! ("linker stdout: {}" , stdout)) } } Err (e) => { let linker_not_found = e . kind () == io :: ErrorKind :: NotFound ; let err = if linker_not_found { sess . dcx () . emit_err (errors :: LinkerNotFound { linker_path , error : e }) } else { sess . dcx () . emit_err (errors :: UnableToExeLinker { linker_path , error : e , command_formatted : format ! ("{cmd:?}") , }) } ; if sess . target . is_like_msvc && linker_not_found { sess . dcx () . emit_note (errors :: MsvcMissingLinker) ; sess . dcx () . emit_note (errors :: CheckInstalledVisualStudio) ; sess . dcx () . emit_note (errors :: InsufficientVSCodeProduct) ; } err . raise_fatal () ; } } match sess . split_debuginfo () { SplitDebuginfo :: Off | SplitDebuginfo :: Unpacked => { } SplitDebuginfo :: Packed if sess . opts . debuginfo == DebugInfo :: None => { } SplitDebuginfo :: Packed if sess . target . is_like_darwin => { let prog = Command :: new ("dsymutil") . arg (out_filename) . output () ; match prog { Ok (prog) => { if ! prog . status . success () { let mut output = prog . stderr . clone () ; output . extend_from_slice (& prog . stdout) ; sess . dcx () . emit_warn (errors :: ProcessingDymutilFailed { status : prog . status , output : escape_string (& output) , }) ; } } Err (error) => sess . dcx () . emit_fatal (errors :: UnableToRunDsymutil { error }) , } } SplitDebuginfo :: Packed if sess . target . is_like_windows => { } SplitDebuginfo :: Packed => link_dwarf_object (sess , codegen_results , out_filename) , } let strip = sess . opts . cg . strip ; if sess . target . is_like_darwin { let stripcmd = "rust-objcopy" ; match (strip , crate_type) { (Strip :: Debuginfo , _) => { strip_with_external_utility (sess , stripcmd , out_filename , & ["--strip-debug"]) } (Strip :: Symbols , CrateType :: Dylib | CrateType :: Cdylib | CrateType :: ProcMacro | CrateType :: Sdylib ,) => strip_with_external_utility (sess , stripcmd , out_filename , & ["--discard-all"]) , (Strip :: Symbols , _) => { strip_with_external_utility (sess , stripcmd , out_filename , & ["--strip-all"]) } (Strip :: None , _) => { } } } if sess . target . is_like_solaris { let stripcmd = if ! sess . host . is_like_solaris { "rust-objcopy" } else { "/usr/bin/strip" } ; match strip { Strip :: Debuginfo => strip_with_external_utility (sess , stripcmd , out_filename , & ["-x"]) , Strip :: Symbols => { } Strip :: None => { } } } if sess . target . is_like_aix { if ! sess . host . is_like_aix { sess . dcx () . emit_warn (errors :: AixStripNotUsed) ; } let stripcmd = "/usr/bin/strip" ; match strip { Strip :: Debuginfo => { strip_with_external_utility (sess , stripcmd , temp_filename , & ["-X32_64" , "-l"]) } Strip :: Symbols => { strip_with_external_utility (sess , stripcmd , temp_filename , & ["-X32_64" , "-r"]) } Strip :: None => { } } } if should_archive { let mut ab = archive_builder_builder . new_archive_builder (sess) ; ab . add_file (temp_filename) ; ab . build (out_filename) ; } }
}

macro_rules! strip_with_external_utility_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function strip_with_external_utility in module {}", module_path!());
    };
}

mkfn!{
    strip_with_external_utility_introspect!();
    fn strip_with_external_utility (sess : & Session , util : & str , out_filename : & Path , options : & [& str]) { let mut cmd = Command :: new (util) ; cmd . args (options) ; let mut new_path = sess . get_tools_search_paths (false) ; if let Some (path) = env :: var_os ("PATH") { new_path . extend (env :: split_paths (& path)) ; } cmd . env ("PATH" , env :: join_paths (new_path) . unwrap ()) ; let prog = cmd . arg (out_filename) . output () ; match prog { Ok (prog) => { if ! prog . status . success () { let mut output = prog . stderr . clone () ; output . extend_from_slice (& prog . stdout) ; sess . dcx () . emit_warn (errors :: StrippingDebugInfoFailed { util , status : prog . status , output : escape_string (& output) , }) ; } } Err (error) => sess . dcx () . emit_fatal (errors :: UnableToRun { util , error }) , } }
}

macro_rules! escape_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function escape_string in module {}", module_path!());
    };
}

mkfn!{
    escape_string_introspect!();
    fn escape_string (s : & [u8]) -> String { match str :: from_utf8 (s) { Ok (s) => s . to_owned () , Err (_) => format ! ("Non-UTF-8 output: {}" , s . escape_ascii ()) , } }
}

macro_rules! escape_linker_output_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function escape_linker_output in module {}", module_path!());
    };
}

mkfn!{
    escape_linker_output_introspect!();
    # [cfg (not (windows))] fn escape_linker_output (s : & [u8] , _flavour : LinkerFlavor) -> String { escape_string (s) }
}

macro_rules! escape_linker_output_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function escape_linker_output in module {}", module_path!());
    };
}

mkfn!{
    escape_linker_output_introspect!();
    # [doc = " If the output of the msvc linker is not UTF-8 and the host is Windows,"] # [doc = " then try to convert the string from the OEM encoding."] # [cfg (windows)] fn escape_linker_output (s : & [u8] , flavour : LinkerFlavor) -> String { if flavour != LinkerFlavor :: Msvc (Lld :: No) { return escape_string (s) ; } match str :: from_utf8 (s) { Ok (s) => return s . to_owned () , Err (_) => match win :: locale_byte_str_to_string (s , win :: oem_code_page ()) { Some (s) => s , None => format ! ("Non-UTF-8 output: {}" , s . escape_ascii ()) , } , } }
}
mkmod!{win, { 
                getname!(win);
                getsrc!(win);
                getpath!(win);
                get_deps!(win);
                get_crates!(win);
                mkinclude!(win);
                mkuse!{use windows :: Win32 :: Globalization :: { CP_OEMCP , GetLocaleInfoEx , LOCALE_IUSEUTF8LEGACYOEMCP , LOCALE_NAME_SYSTEM_DEFAULT , LOCALE_RETURN_NUMBER , MB_ERR_INVALID_CHARS , MultiByteToWideChar , } ;}

macro_rules! oem_code_page_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function oem_code_page in module {}", module_path!());
    };
}

mkfn!{
    oem_code_page_introspect!();
    # [doc = " Get the Windows system OEM code page. This is most notably the code page"] # [doc = " used for link.exe's output."] pub (super) fn oem_code_page () -> u32 { unsafe { let mut cp : u32 = 0 ; let len = size_of :: < u32 > () / size_of :: < u16 > () ; let data = std :: slice :: from_raw_parts_mut (& mut cp as * mut u32 as * mut u16 , len) ; let len_written = GetLocaleInfoEx (LOCALE_NAME_SYSTEM_DEFAULT , LOCALE_IUSEUTF8LEGACYOEMCP | LOCALE_RETURN_NUMBER , Some (data) ,) ; if len_written as usize == len { cp } else { CP_OEMCP } } }
}

macro_rules! locale_byte_str_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function locale_byte_str_to_string in module {}", module_path!());
    };
}

mkfn!{
    locale_byte_str_to_string_introspect!();
    # [doc = " Try to convert a multi-byte string to a UTF-8 string using the given code page"] # [doc = " The string does not need to be null terminated."] # [doc = ""] # [doc = " This is implemented as a wrapper around `MultiByteToWideChar`."] # [doc = " See <https://learn.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-multibytetowidechar>"] # [doc = ""] # [doc = " It will fail if the multi-byte string is longer than `i32::MAX` or if it contains"] # [doc = " any invalid bytes for the expected encoding."] pub (super) fn locale_byte_str_to_string (s : & [u8] , code_page : u32) -> Option < String > { if s . len () > isize :: MAX as usize { return None ; } let flags = MB_ERR_INVALID_CHARS ; let mut len = unsafe { MultiByteToWideChar (code_page , flags , s , None) } ; if len > 0 { let mut utf16 = vec ! [0 ; len as usize] ; len = unsafe { MultiByteToWideChar (code_page , flags , s , Some (& mut utf16)) } ; if len > 0 { return utf16 . get (.. len as usize) . map (String :: from_utf16_lossy) ; } } None }
} 
            }}

macro_rules! add_sanitizer_libraries_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_sanitizer_libraries in module {}", module_path!());
    };
}

mkfn!{
    add_sanitizer_libraries_introspect!();
    fn add_sanitizer_libraries (sess : & Session , flavor : LinkerFlavor , crate_type : CrateType , linker : & mut dyn Linker ,) { if sess . target . is_like_android { return ; } if sess . opts . unstable_opts . external_clangrt { return ; } if matches ! (crate_type , CrateType :: Rlib | CrateType :: Staticlib) { return ; } if matches ! (crate_type , CrateType :: Dylib | CrateType :: Cdylib | CrateType :: ProcMacro | CrateType :: Sdylib) && ! (sess . target . is_like_darwin || sess . target . is_like_msvc) { return ; } let sanitizer = sess . opts . unstable_opts . sanitizer ; if sanitizer . contains (SanitizerSet :: ADDRESS) { link_sanitizer_runtime (sess , flavor , linker , "asan") ; } if sanitizer . contains (SanitizerSet :: DATAFLOW) { link_sanitizer_runtime (sess , flavor , linker , "dfsan") ; } if sanitizer . contains (SanitizerSet :: LEAK) && ! sanitizer . contains (SanitizerSet :: ADDRESS) && ! sanitizer . contains (SanitizerSet :: HWADDRESS) { link_sanitizer_runtime (sess , flavor , linker , "lsan") ; } if sanitizer . contains (SanitizerSet :: MEMORY) { link_sanitizer_runtime (sess , flavor , linker , "msan") ; } if sanitizer . contains (SanitizerSet :: THREAD) { link_sanitizer_runtime (sess , flavor , linker , "tsan") ; } if sanitizer . contains (SanitizerSet :: HWADDRESS) { link_sanitizer_runtime (sess , flavor , linker , "hwasan") ; } if sanitizer . contains (SanitizerSet :: SAFESTACK) { link_sanitizer_runtime (sess , flavor , linker , "safestack") ; } }
}

macro_rules! link_sanitizer_runtime_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function link_sanitizer_runtime in module {}", module_path!());
    };
}

mkfn!{
    link_sanitizer_runtime_introspect!();
    fn link_sanitizer_runtime (sess : & Session , flavor : LinkerFlavor , linker : & mut dyn Linker , name : & str ,) { fn find_sanitizer_runtime (sess : & Session , filename : & str) -> PathBuf { let path = sess . target_tlib_path . dir . join (filename) ; if path . exists () { sess . target_tlib_path . dir . clone () } else { filesearch :: make_target_lib_path (& sess . opts . sysroot . default , sess . opts . target_triple . tuple () ,) } } let channel = option_"dev" . map (| channel | format ! ("-{channel}")) . unwrap_or_default () ; if sess . target . is_like_darwin { let filename = format ! ("rustc{channel}_rt.{name}") ; let path = find_sanitizer_runtime (sess , & filename) ; let rpath = path . to_str () . expect ("non-utf8 component in path") ; linker . link_args (& ["-rpath" , rpath]) ; linker . link_dylib_by_name (& filename , false , true) ; } else if sess . target . is_like_msvc && flavor == LinkerFlavor :: Msvc (Lld :: No) && name == "asan" { linker . link_arg ("/INFERASANLIBS") ; } else { let filename = format ! ("librustc{channel}_rt.{name}.a") ; let path = find_sanitizer_runtime (sess , & filename) . join (& filename) ; linker . link_staticlib_by_path (& path , true) ; } }
}

macro_rules! ignored_for_lto_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ignored_for_lto in module {}", module_path!());
    };
}

mkfn!{
    ignored_for_lto_introspect!();
    # [doc = " Returns a boolean indicating whether the specified crate should be ignored"] # [doc = " during LTO."] # [doc = ""] # [doc = " Crates ignored during LTO are not lumped together in the \"massive object"] # [doc = " file\" that we create and are linked in their normal rlib states. See"] # [doc = " comments below for what crates do not participate in LTO."] # [doc = ""] # [doc = " It's unusual for a crate to not participate in LTO. Typically only"] # [doc = " compiler-specific and unstable crates have a reason to not participate in"] # [doc = " LTO."] pub fn ignored_for_lto (sess : & Session , info : & CrateInfo , cnum : CrateNum) -> bool { ! sess . target . no_builtins && (info . compiler_builtins == Some (cnum) || info . is_no_builtins . contains (& cnum)) }
}

macro_rules! linker_and_flavor_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function linker_and_flavor in module {}", module_path!());
    };
}

mkfn!{
    linker_and_flavor_introspect!();
    # [doc = " This functions tries to determine the appropriate linker (and corresponding LinkerFlavor) to use"] pub fn linker_and_flavor (sess : & Session) -> (PathBuf , LinkerFlavor) { fn infer_from (sess : & Session , linker : Option < PathBuf > , flavor : Option < LinkerFlavor > , features : LinkerFeaturesCli ,) -> Option < (PathBuf , LinkerFlavor) > { let flavor = flavor . map (| flavor | adjust_flavor_to_features (flavor , features)) ; match (linker , flavor) { (Some (linker) , Some (flavor)) => Some ((linker , flavor)) , (None , Some (flavor)) => Some ((PathBuf :: from (match flavor { LinkerFlavor :: Gnu (Cc :: Yes , _) | LinkerFlavor :: Darwin (Cc :: Yes , _) | LinkerFlavor :: WasmLld (Cc :: Yes) | LinkerFlavor :: Unix (Cc :: Yes) => { if cfg ! (any (target_os = "solaris" , target_os = "illumos")) { "gcc" } else { "cc" } } LinkerFlavor :: Gnu (_ , Lld :: Yes) | LinkerFlavor :: Darwin (_ , Lld :: Yes) | LinkerFlavor :: WasmLld (..) | LinkerFlavor :: Msvc (Lld :: Yes) => "lld" , LinkerFlavor :: Gnu (..) | LinkerFlavor :: Darwin (..) | LinkerFlavor :: Unix (..) => { "ld" } LinkerFlavor :: Msvc (..) => "link.exe" , LinkerFlavor :: EmCc => { if cfg ! (windows) { "emcc.bat" } else { "emcc" } } LinkerFlavor :: Bpf => "bpf-linker" , LinkerFlavor :: Llbc => "llvm-bitcode-linker" , LinkerFlavor :: Ptx => "rust-ptx-linker" , }) , flavor ,)) , (Some (linker) , None) => { let stem = linker . file_stem () . and_then (| stem | stem . to_str ()) . unwrap_or_else (| | { sess . dcx () . emit_fatal (errors :: LinkerFileStem) ; }) ; let flavor = sess . target . linker_flavor . with_linker_hints (stem) ; let flavor = adjust_flavor_to_features (flavor , features) ; Some ((linker , flavor)) } (None , None) => None , } } fn adjust_flavor_to_features (flavor : LinkerFlavor , features : LinkerFeaturesCli ,) -> LinkerFlavor { if features . enabled . contains (LinkerFeatures :: LLD) { flavor . with_lld_enabled () } else if features . disabled . contains (LinkerFeatures :: LLD) { flavor . with_lld_disabled () } else { flavor } } let features = sess . opts . cg . linker_features ; let linker_flavor = match sess . opts . cg . linker_flavor { Some (LinkerFlavorCli :: Llbc) => Some (LinkerFlavor :: Llbc) , Some (LinkerFlavorCli :: Ptx) => Some (LinkerFlavor :: Ptx) , _ => sess . opts . cg . linker_flavor . map (| flavor | sess . target . linker_flavor . with_cli_hints (flavor)) , } ; if let Some (ret) = infer_from (sess , sess . opts . cg . linker . clone () , linker_flavor , features) { return ret ; } if let Some (ret) = infer_from (sess , sess . target . linker . as_deref () . map (PathBuf :: from) , Some (sess . target . linker_flavor) , features ,) { return ret ; } bug ! ("Not enough information provided to determine how to invoke the linker") ; }
}

macro_rules! preserve_objects_for_their_debuginfo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function preserve_objects_for_their_debuginfo in module {}", module_path!());
    };
}

mkfn!{
    preserve_objects_for_their_debuginfo_introspect!();
    # [doc = " Returns a pair of boolean indicating whether we should preserve the object and"] # [doc = " dwarf object files on the filesystem for their debug information. This is often"] # [doc = " useful with split-dwarf like schemes."] fn preserve_objects_for_their_debuginfo (sess : & Session) -> (bool , bool) { if sess . opts . debuginfo == config :: DebugInfo :: None { return (false , false) ; } match (sess . split_debuginfo () , sess . opts . unstable_opts . split_dwarf_kind) { (SplitDebuginfo :: Off , _) => (false , false) , (SplitDebuginfo :: Packed , _) => (false , false) , (SplitDebuginfo :: Unpacked , _) if ! sess . target_can_use_split_dwarf () => (true , false) , (SplitDebuginfo :: Unpacked , SplitDwarfKind :: Single) => (true , false) , (SplitDebuginfo :: Unpacked , SplitDwarfKind :: Split) => (false , true) , } }
}
mkitem!{mkenum!{# [derive (PartialEq)] enum RlibFlavor { Normal , StaticlibBase , }}}

macro_rules! print_native_static_libs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function print_native_static_libs in module {}", module_path!());
    };
}

mkfn!{
    print_native_static_libs_introspect!();
    fn print_native_static_libs (sess : & Session , out : & OutFileName , all_native_libs : & [NativeLib] , all_rust_dylibs : & [& Path] ,) { let mut lib_args : Vec < _ > = all_native_libs . iter () . filter (| l | relevant_lib (sess , l)) . filter_map (| lib | { let name = lib . name ; match lib . kind { NativeLibKind :: Static { bundle : Some (false) , .. } | NativeLibKind :: Dylib { .. } | NativeLibKind :: Unspecified => { let verbatim = lib . verbatim ; if sess . target . is_like_msvc { let (prefix , suffix) = sess . staticlib_components (verbatim) ; Some (format ! ("{prefix}{name}{suffix}")) } else if sess . target . linker_flavor . is_gnu () { Some (format ! ("-l{}{}" , if verbatim { ":" } else { "" } , name)) } else { Some (format ! ("-l{name}")) } } NativeLibKind :: Framework { .. } => { Some (format ! ("-framework {name}")) } NativeLibKind :: Static { bundle : None | Some (true) , .. } | NativeLibKind :: LinkArg | NativeLibKind :: WasmImportModule | NativeLibKind :: RawDylib => None , } }) . dedup () . collect () ; for path in all_rust_dylibs { let parent = path . parent () ; if let Some (dir) = parent { let dir = fix_windows_verbatim_for_gcc (dir) ; if sess . target . is_like_msvc { let mut arg = String :: from ("/LIBPATH:") ; arg . push_str (& dir . display () . to_string ()) ; lib_args . push (arg) ; } else { lib_args . push ("-L" . to_owned ()) ; lib_args . push (dir . display () . to_string ()) ; } } let stem = path . file_stem () . unwrap () . to_str () . unwrap () ; let lib = if let Some (lib) = stem . strip_prefix ("lib") && ! sess . target . is_like_windows { lib } else { stem } ; let path = parent . unwrap_or_else (| | Path :: new ("")) ; if sess . target . is_like_msvc { let name = format ! ("{lib}.dll.lib") ; if path . join (& name) . exists () { lib_args . push (name) ; } } else { lib_args . push (format ! ("-l{lib}")) ; } } match out { OutFileName :: Real (path) => { out . overwrite (& lib_args . join (" ") , sess) ; sess . dcx () . emit_note (errors :: StaticLibraryNativeArtifactsToFile { path }) ; } OutFileName :: Stdout => { sess . dcx () . emit_note (errors :: StaticLibraryNativeArtifacts) ; sess . dcx () . note (format ! ("native-static-libs: {}" , lib_args . join (" "))) ; } } }
}

macro_rules! get_object_file_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_object_file_path in module {}", module_path!());
    };
}

mkfn!{
    get_object_file_path_introspect!();
    fn get_object_file_path (sess : & Session , name : & str , self_contained : bool) -> PathBuf { let file_path = sess . target_tlib_path . dir . join (name) ; if file_path . exists () { return file_path ; } if self_contained { let file_path = sess . target_tlib_path . dir . join ("self-contained") . join (name) ; if file_path . exists () { return file_path ; } } for search_path in sess . target_filesearch () . search_paths (PathKind :: Native) { let file_path = search_path . dir . join (name) ; if file_path . exists () { return file_path ; } } PathBuf :: from (name) }
}

macro_rules! exec_linker_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exec_linker in module {}", module_path!());
    };
}

mkfn!{
    exec_linker_introspect!();
    fn exec_linker (sess : & Session , cmd : & Command , out_filename : & Path , flavor : LinkerFlavor , tmpdir : & Path ,) -> io :: Result < Output > { if ! cmd . very_likely_to_exceed_some_spawn_limit () { match cmd . command () . stdout (Stdio :: piped ()) . stderr (Stdio :: piped ()) . spawn () { Ok (child) => { let output = child . wait_with_output () ; flush_linked_file (& output , out_filename) ? ; return output ; } Err (ref e) if command_line_too_big (e) => { info ! ("command line to linker was too big: {}" , e) ; } Err (e) => return Err (e) , } } info ! ("falling back to passing arguments to linker via an @-file") ; let mut cmd2 = cmd . clone () ; let mut args = String :: new () ; for arg in cmd2 . take_args () { args . push_str (& Escape { arg : arg . to_str () . unwrap () , is_like_msvc : sess . target . is_like_msvc || (cfg ! (windows) && flavor . uses_lld () && ! flavor . uses_cc ()) , } . to_string () ,) ; args . push ('\n') ; } let file = tmpdir . join ("linker-arguments") ; let bytes = if sess . target . is_like_msvc { let mut out = Vec :: with_capacity ((1 + args . len ()) * 2) ; for c in std :: iter :: once (0xFEFF) . chain (args . encode_utf16 ()) { out . push (c as u8) ; out . push ((c >> 8) as u8) ; } out } else { args . into_bytes () } ; fs :: write (& file , & bytes) ? ; cmd2 . arg (format ! ("@{}" , file . display ())) ; info ! ("invoking linker {:?}" , cmd2) ; let output = cmd2 . output () ; flush_linked_file (& output , out_filename) ? ; return output ; # [cfg (not (windows))] fn flush_linked_file (_ : & io :: Result < Output > , _ : & Path) -> io :: Result < () > { Ok (()) } # [cfg (windows)] fn flush_linked_file (command_output : & io :: Result < Output > , out_filename : & Path ,) -> io :: Result < () > { if let & Ok (ref out) = command_output { if out . status . success () { if let Ok (of) = fs :: OpenOptions :: new () . write (true) . open (out_filename) { of . sync_all () ? ; } } } Ok (()) } # [cfg (unix)] fn command_line_too_big (err : & io :: Error) -> bool { err . raw_os_error () == Some (:: libc :: E2BIG) } # [cfg (windows)] fn command_line_too_big (err : & io :: Error) -> bool { const ERROR_FILENAME_EXCED_RANGE : i32 = 206 ; err . raw_os_error () == Some (ERROR_FILENAME_EXCED_RANGE) } # [cfg (not (any (unix , windows)))] fn command_line_too_big (_ : & io :: Error) -> bool { false } struct Escape < 'a > { arg : & 'a str , is_like_msvc : bool , } impl < 'a > fmt :: Display for Escape < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . is_like_msvc { write ! (f , "\"") ? ; for c in self . arg . chars () { match c { '"' => write ! (f , "\\{c}") ? , c => write ! (f , "{c}") ? , } } write ! (f , "\"") ? ; } else { for c in self . arg . chars () { match c { '\\' | ' ' => write ! (f , "\\{c}") ? , c => write ! (f , "{c}") ? , } } } Ok (()) } } }
}

macro_rules! link_output_kind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function link_output_kind in module {}", module_path!());
    };
}

mkfn!{
    link_output_kind_introspect!();
    fn link_output_kind (sess : & Session , crate_type : CrateType) -> LinkOutputKind { let kind = match (crate_type , sess . crt_static (Some (crate_type)) , sess . relocation_model ()) { (CrateType :: Executable , _ , _) if sess . is_wasi_reactor () => LinkOutputKind :: WasiReactorExe , (CrateType :: Executable , false , RelocModel :: Pic | RelocModel :: Pie) => { LinkOutputKind :: DynamicPicExe } (CrateType :: Executable , false , _) => LinkOutputKind :: DynamicNoPicExe , (CrateType :: Executable , true , RelocModel :: Pic | RelocModel :: Pie) => { LinkOutputKind :: StaticPicExe } (CrateType :: Executable , true , _) => LinkOutputKind :: StaticNoPicExe , (_ , true , _) => LinkOutputKind :: StaticDylib , (_ , false , _) => LinkOutputKind :: DynamicDylib , } ; let opts = & sess . target ; let pic_exe_supported = opts . position_independent_executables ; let static_pic_exe_supported = opts . static_position_independent_executables ; let static_dylib_supported = opts . crt_static_allows_dylibs ; match kind { LinkOutputKind :: DynamicPicExe if ! pic_exe_supported => LinkOutputKind :: DynamicNoPicExe , LinkOutputKind :: StaticPicExe if ! static_pic_exe_supported => LinkOutputKind :: StaticNoPicExe , LinkOutputKind :: StaticDylib if ! static_dylib_supported => LinkOutputKind :: DynamicDylib , _ => kind , } }
}

macro_rules! detect_self_contained_mingw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function detect_self_contained_mingw in module {}", module_path!());
    };
}

mkfn!{
    detect_self_contained_mingw_introspect!();
    fn detect_self_contained_mingw (sess : & Session , linker : & Path) -> bool { if linker == Path :: new ("rust-lld") { return true ; } let linker_with_extension = if cfg ! (windows) && linker . extension () . is_none () { linker . with_extension ("exe") } else { linker . to_path_buf () } ; for dir in env :: split_paths (& env :: var_os ("PATH") . unwrap_or_default ()) { let full_path = dir . join (& linker_with_extension) ; if full_path . is_file () && ! full_path . starts_with (sess . opts . sysroot . path ()) { return false ; } } true }
}

macro_rules! self_contained_components_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function self_contained_components in module {}", module_path!());
    };
}

mkfn!{
    self_contained_components_introspect!();
    # [doc = " Various toolchain components used during linking are used from rustc distribution"] # [doc = " instead of being found somewhere on the host system."] # [doc = " We only provide such support for a very limited number of targets."] fn self_contained_components (sess : & Session , crate_type : CrateType , linker : & Path ,) -> LinkSelfContainedComponents { let self_contained = if let Some (self_contained) = sess . opts . cg . link_self_contained . explicitly_set { if sess . target . link_self_contained . is_disabled () { sess . dcx () . emit_err (errors :: UnsupportedLinkSelfContained) ; } self_contained } else { match sess . target . link_self_contained { LinkSelfContainedDefault :: False => false , LinkSelfContainedDefault :: True => true , LinkSelfContainedDefault :: WithComponents (components) => { return components ; } LinkSelfContainedDefault :: InferredForMusl => sess . crt_static (Some (crate_type)) , LinkSelfContainedDefault :: InferredForMingw => { sess . host == sess . target && sess . target . vendor != "uwp" && detect_self_contained_mingw (sess , linker) } } } ; if self_contained { LinkSelfContainedComponents :: all () } else { LinkSelfContainedComponents :: empty () } }
}

macro_rules! add_pre_link_objects_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_pre_link_objects in module {}", module_path!());
    };
}

mkfn!{
    add_pre_link_objects_introspect!();
    # [doc = " Add pre-link object files defined by the target spec."] fn add_pre_link_objects (cmd : & mut dyn Linker , sess : & Session , flavor : LinkerFlavor , link_output_kind : LinkOutputKind , self_contained : bool ,) { let opts = & sess . target ; let empty = Default :: default () ; let objects = if self_contained { & opts . pre_link_objects_self_contained } else if ! (sess . target . os == "fuchsia" && matches ! (flavor , LinkerFlavor :: Gnu (Cc :: Yes , _))) { & opts . pre_link_objects } else { & empty } ; for obj in objects . get (& link_output_kind) . iter () . copied () . flatten () { cmd . add_object (& get_object_file_path (sess , obj , self_contained)) ; } }
}

macro_rules! add_post_link_objects_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_post_link_objects in module {}", module_path!());
    };
}

mkfn!{
    add_post_link_objects_introspect!();
    # [doc = " Add post-link object files defined by the target spec."] fn add_post_link_objects (cmd : & mut dyn Linker , sess : & Session , link_output_kind : LinkOutputKind , self_contained : bool ,) { let objects = if self_contained { & sess . target . post_link_objects_self_contained } else { & sess . target . post_link_objects } ; for obj in objects . get (& link_output_kind) . iter () . copied () . flatten () { cmd . add_object (& get_object_file_path (sess , obj , self_contained)) ; } }
}

macro_rules! add_pre_link_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_pre_link_args in module {}", module_path!());
    };
}

mkfn!{
    add_pre_link_args_introspect!();
    # [doc = " Add arbitrary \"pre-link\" args defined by the target spec or from command line."] # [doc = " FIXME: Determine where exactly these args need to be inserted."] fn add_pre_link_args (cmd : & mut dyn Linker , sess : & Session , flavor : LinkerFlavor) { if let Some (args) = sess . target . pre_link_args . get (& flavor) { cmd . verbatim_args (args . iter () . map (Deref :: deref)) ; } cmd . verbatim_args (& sess . opts . unstable_opts . pre_link_args) ; }
}

macro_rules! add_link_script_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_link_script in module {}", module_path!());
    };
}

mkfn!{
    add_link_script_introspect!();
    # [doc = " Add a link script embedded in the target, if applicable."] fn add_link_script (cmd : & mut dyn Linker , sess : & Session , tmpdir : & Path , crate_type : CrateType) { match (crate_type , & sess . target . link_script) { (CrateType :: Cdylib | CrateType :: Executable , Some (script)) => { if ! sess . target . linker_flavor . is_gnu () { sess . dcx () . emit_fatal (errors :: LinkScriptUnavailable) ; } let file_name = ["rustc" , & sess . target . llvm_target , "linkfile.ld"] . join ("-") ; let path = tmpdir . join (file_name) ; if let Err (error) = fs :: write (& path , script . as_ref ()) { sess . dcx () . emit_fatal (errors :: LinkScriptWriteFailure { path , error }) ; } cmd . link_arg ("--script") . link_arg (path) ; } _ => { } } }
}

macro_rules! add_user_defined_link_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_user_defined_link_args in module {}", module_path!());
    };
}

mkfn!{
    add_user_defined_link_args_introspect!();
    # [doc = " Add arbitrary \"user defined\" args defined from command line."] # [doc = " FIXME: Determine where exactly these args need to be inserted."] fn add_user_defined_link_args (cmd : & mut dyn Linker , sess : & Session) { cmd . verbatim_args (& sess . opts . cg . link_args) ; }
}

macro_rules! add_late_link_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_late_link_args in module {}", module_path!());
    };
}

mkfn!{
    add_late_link_args_introspect!();
    # [doc = " Add arbitrary \"late link\" args defined by the target spec."] # [doc = " FIXME: Determine where exactly these args need to be inserted."] fn add_late_link_args (cmd : & mut dyn Linker , sess : & Session , flavor : LinkerFlavor , crate_type : CrateType , codegen_results : & CodegenResults ,) { let any_dynamic_crate = crate_type == CrateType :: Dylib || crate_type == CrateType :: Sdylib || codegen_results . crate_info . dependency_formats . iter () . any (| (ty , list) | { * ty == crate_type && list . iter () . any (| & linkage | linkage == Linkage :: Dynamic) }) ; if any_dynamic_crate { if let Some (args) = sess . target . late_link_args_dynamic . get (& flavor) { cmd . verbatim_args (args . iter () . map (Deref :: deref)) ; } } else if let Some (args) = sess . target . late_link_args_static . get (& flavor) { cmd . verbatim_args (args . iter () . map (Deref :: deref)) ; } if let Some (args) = sess . target . late_link_args . get (& flavor) { cmd . verbatim_args (args . iter () . map (Deref :: deref)) ; } }
}

macro_rules! add_post_link_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_post_link_args in module {}", module_path!());
    };
}

mkfn!{
    add_post_link_args_introspect!();
    # [doc = " Add arbitrary \"post-link\" args defined by the target spec."] # [doc = " FIXME: Determine where exactly these args need to be inserted."] fn add_post_link_args (cmd : & mut dyn Linker , sess : & Session , flavor : LinkerFlavor) { if let Some (args) = sess . target . post_link_args . get (& flavor) { cmd . verbatim_args (args . iter () . map (Deref :: deref)) ; } }
}

macro_rules! add_linked_symbol_object_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_linked_symbol_object in module {}", module_path!());
    };
}

mkfn!{
    add_linked_symbol_object_introspect!();
    # [doc = " Add a synthetic object file that contains reference to all symbols that we want to expose to"] # [doc = " the linker."] # [doc = ""] # [doc = " Background: we implement rlibs as static library (archives). Linkers treat archives"] # [doc = " differently from object files: all object files participate in linking, while archives will"] # [doc = " only participate in linking if they can satisfy at least one undefined reference (version"] # [doc = " scripts doesn't count). This causes `#[no_mangle]` or `#[used]` items to be ignored by the"] # [doc = " linker, and since they never participate in the linking, using `KEEP` in the linker scripts"] # [doc = " can't keep them either. This causes #47384."] # [doc = ""] # [doc = " To keep them around, we could use `--whole-archive`, `-force_load` and equivalents to force rlib"] # [doc = " to participate in linking like object files, but this proves to be expensive (#93791). Therefore"] # [doc = " we instead just introduce an undefined reference to them. This could be done by `-u` command"] # [doc = " line option to the linker or `EXTERN(...)` in linker scripts, however they does not only"] # [doc = " introduce an undefined reference, but also make them the GC roots, preventing `--gc-sections`"] # [doc = " from removing them, and this is especially problematic for embedded programming where every"] # [doc = " byte counts."] # [doc = ""] # [doc = " This method creates a synthetic object file, which contains undefined references to all symbols"] # [doc = " that are necessary for the linking. They are only present in symbol table but not actually"] # [doc = " used in any sections, so the linker will therefore pick relevant rlibs for linking, but"] # [doc = " unused `#[no_mangle]` or `#[used(compiler)]` can still be discard by GC sections."] # [doc = ""] # [doc = " There's a few internal crates in the standard library (aka libcore and"] # [doc = " libstd) which actually have a circular dependence upon one another. This"] # [doc = " currently arises through \"weak lang items\" where libcore requires things"] # [doc = " like `rust_begin_unwind` but libstd ends up defining it. To get this"] # [doc = " circular dependence to work correctly we declare some of these things"] # [doc = " in this synthetic object."] fn add_linked_symbol_object (cmd : & mut dyn Linker , sess : & Session , tmpdir : & Path , symbols : & [(String , SymbolExportKind)] ,) { if symbols . is_empty () { return ; } let Some (mut file) = super :: metadata :: create_object_file (sess) else { return ; } ; if file . format () == object :: BinaryFormat :: Coff { file . add_section (Vec :: new () , ".text" . into () , object :: SectionKind :: Text) ; file . set_mangling (object :: write :: Mangling :: None) ; } if file . format () == object :: BinaryFormat :: MachO { file . set_subsections_via_symbols () ; } let ld64_section_helper = if file . format () == object :: BinaryFormat :: MachO { Some (file . add_section (file . segment_name (object :: write :: StandardSegment :: Data) . to_vec () , "__data" . into () , object :: SectionKind :: Data ,)) } else { None } ; for (sym , kind) in symbols . iter () { let symbol = file . add_symbol (object :: write :: Symbol { name : sym . clone () . into () , value : 0 , size : 0 , kind : match kind { SymbolExportKind :: Text => object :: SymbolKind :: Text , SymbolExportKind :: Data => object :: SymbolKind :: Data , SymbolExportKind :: Tls => object :: SymbolKind :: Tls , } , scope : object :: SymbolScope :: Unknown , weak : false , section : object :: write :: SymbolSection :: Undefined , flags : object :: SymbolFlags :: None , }) ; if let Some (section) = ld64_section_helper { apple :: add_data_and_relocation (& mut file , section , symbol , & sess . target , * kind) . expect ("failed adding relocation") ; } } let path = tmpdir . join ("symbols.o") ; let result = std :: fs :: write (& path , file . write () . unwrap ()) ; if let Err (error) = result { sess . dcx () . emit_fatal (errors :: FailedToWrite { path , error }) ; } cmd . add_object (& path) ; }
}

macro_rules! add_local_crate_regular_objects_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_local_crate_regular_objects in module {}", module_path!());
    };
}

mkfn!{
    add_local_crate_regular_objects_introspect!();
    # [doc = " Add object files containing code from the current crate."] fn add_local_crate_regular_objects (cmd : & mut dyn Linker , codegen_results : & CodegenResults) { for obj in codegen_results . modules . iter () . filter_map (| m | m . object . as_ref ()) { cmd . add_object (obj) ; } }
}

macro_rules! add_local_crate_allocator_objects_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_local_crate_allocator_objects in module {}", module_path!());
    };
}

mkfn!{
    add_local_crate_allocator_objects_introspect!();
    # [doc = " Add object files for allocator code linked once for the whole crate tree."] fn add_local_crate_allocator_objects (cmd : & mut dyn Linker , codegen_results : & CodegenResults , crate_type : CrateType ,) { if needs_allocator_shim_for_linking (& codegen_results . crate_info . dependency_formats , crate_type) { if let Some (obj) = codegen_results . allocator_module . as_ref () . and_then (| m | m . object . as_ref ()) { cmd . add_object (obj) ; } } }
}

macro_rules! add_local_crate_metadata_objects_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_local_crate_metadata_objects in module {}", module_path!());
    };
}

mkfn!{
    add_local_crate_metadata_objects_introspect!();
    # [doc = " Add object files containing metadata for the current crate."] fn add_local_crate_metadata_objects (cmd : & mut dyn Linker , sess : & Session , archive_builder_builder : & dyn ArchiveBuilderBuilder , crate_type : CrateType , tmpdir : & Path , codegen_results : & CodegenResults , metadata : & EncodedMetadata ,) { if matches ! (crate_type , CrateType :: Dylib | CrateType :: ProcMacro) { let data = archive_builder_builder . create_dylib_metadata_wrapper (sess , & metadata , & codegen_results . crate_info . metadata_symbol ,) ; let obj = emit_wrapper_file (sess , & data , tmpdir , "rmeta.o") ; cmd . add_object (& obj) ; } }
}

macro_rules! add_library_search_dirs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_library_search_dirs in module {}", module_path!());
    };
}

mkfn!{
    add_library_search_dirs_introspect!();
    # [doc = " Add sysroot and other globally set directories to the directory search list."] fn add_library_search_dirs (cmd : & mut dyn Linker , sess : & Session , self_contained_components : LinkSelfContainedComponents , apple_sdk_root : Option < & Path > ,) { if ! sess . opts . unstable_opts . link_native_libraries { return ; } let fallback = Some (NativeLibSearchFallback { self_contained_components , apple_sdk_root }) ; let _ = walk_native_lib_search_dirs (sess , fallback , | dir , is_framework | { if is_framework { cmd . framework_path (dir) ; } else { cmd . include_path (& fix_windows_verbatim_for_gcc (dir)) ; } ControlFlow :: < () > :: Continue (()) }) ; }
}

macro_rules! add_relro_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_relro_args in module {}", module_path!());
    };
}

mkfn!{
    add_relro_args_introspect!();
    # [doc = " Add options making relocation sections in the produced ELF files read-only"] # [doc = " and suppressing lazy binding."] fn add_relro_args (cmd : & mut dyn Linker , sess : & Session) { match sess . opts . cg . relro_level . unwrap_or (sess . target . relro_level) { RelroLevel :: Full => cmd . full_relro () , RelroLevel :: Partial => cmd . partial_relro () , RelroLevel :: Off => cmd . no_relro () , RelroLevel :: None => { } } }
}

macro_rules! add_rpath_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_rpath_args in module {}", module_path!());
    };
}

mkfn!{
    add_rpath_args_introspect!();
    # [doc = " Add library search paths used at runtime by dynamic linkers."] fn add_rpath_args (cmd : & mut dyn Linker , sess : & Session , codegen_results : & CodegenResults , out_filename : & Path ,) { if ! sess . target . has_rpath { return ; } if sess . opts . cg . rpath { let libs = codegen_results . crate_info . used_crates . iter () . filter_map (| cnum | { codegen_results . crate_info . used_crate_source [cnum] . dylib . as_ref () . map (| (path , _) | & * * path) }) . collect :: < Vec < _ > > () ; let rpath_config = RPathConfig { libs : & * libs , out_filename : out_filename . to_path_buf () , is_like_darwin : sess . target . is_like_darwin , linker_is_gnu : sess . target . linker_flavor . is_gnu () , } ; cmd . link_args (& rpath :: get_rpath_linker_args (& rpath_config)) ; } }
}

macro_rules! linker_with_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function linker_with_args in module {}", module_path!());
    };
}

mkfn!{
    linker_with_args_introspect!();
    # [doc = " Produce the linker command line containing linker path and arguments."] # [doc = ""] # [doc = " When comments in the function say \"order-(in)dependent\" they mean order-dependence between"] # [doc = " options and libraries/object files. For example `--whole-archive` (order-dependent) applies"] # [doc = " to specific libraries passed after it, and `-o` (output file, order-independent) applies"] # [doc = " to the linking process as a whole."] # [doc = " Order-independent options may still override each other in order-dependent fashion,"] # [doc = " e.g `--foo=yes --foo=no` may be equivalent to `--foo=no`."] fn linker_with_args (path : & Path , flavor : LinkerFlavor , sess : & Session , archive_builder_builder : & dyn ArchiveBuilderBuilder , crate_type : CrateType , tmpdir : & Path , out_filename : & Path , codegen_results : & CodegenResults , metadata : & EncodedMetadata , self_contained_components : LinkSelfContainedComponents ,) -> Command { let self_contained_crt_objects = self_contained_components . is_crt_objects_enabled () ; let cmd = & mut * super :: linker :: get_linker (sess , path , flavor , self_contained_components . are_any_components_enabled () , & codegen_results . crate_info . target_cpu ,) ; let link_output_kind = link_output_kind (sess , crate_type) ; cmd . export_symbols (tmpdir , crate_type , & codegen_results . crate_info . exported_symbols [& crate_type] ,) ; add_pre_link_args (cmd , sess , flavor) ; add_pre_link_objects (cmd , sess , flavor , link_output_kind , self_contained_crt_objects) ; add_linked_symbol_object (cmd , sess , tmpdir , & codegen_results . crate_info . linked_symbols [& crate_type] ,) ; add_sanitizer_libraries (sess , flavor , crate_type , cmd) ; add_local_crate_regular_objects (cmd , codegen_results) ; add_local_crate_metadata_objects (cmd , sess , archive_builder_builder , crate_type , tmpdir , codegen_results , metadata ,) ; add_local_crate_allocator_objects (cmd , codegen_results , crate_type) ; cmd . add_as_needed () ; add_local_native_libraries (cmd , sess , archive_builder_builder , codegen_results , tmpdir , link_output_kind ,) ; add_upstream_rust_crates (cmd , sess , archive_builder_builder , codegen_results , crate_type , tmpdir , link_output_kind ,) ; add_upstream_native_libraries (cmd , sess , archive_builder_builder , codegen_results , tmpdir , link_output_kind ,) ; let raw_dylib_dir = tmpdir . join ("raw-dylibs") ; if sess . target . binary_format == BinaryFormat :: Elf { if let Err (error) = fs :: create_dir (& raw_dylib_dir) { sess . dcx () . emit_fatal (errors :: CreateTempDir { error }) } cmd . include_path (& raw_dylib_dir) ; } if sess . target . is_like_windows { for output_path in raw_dylib :: create_raw_dylib_dll_import_libs (sess , archive_builder_builder , codegen_results . crate_info . used_libraries . iter () , tmpdir , true ,) { cmd . add_object (& output_path) ; } } else { for link_path in raw_dylib :: create_raw_dylib_elf_stub_shared_objects (sess , codegen_results . crate_info . used_libraries . iter () , & raw_dylib_dir ,) { cmd . link_dylib_by_name (& link_path , true , false) ; } } let dependency_linkage = codegen_results . crate_info . dependency_formats . get (& crate_type) . expect ("failed to find crate type in dependency format list") ; # [allow (rustc :: potential_query_instability)] let mut native_libraries_from_nonstatics = codegen_results . crate_info . native_libraries . iter () . filter_map (| (& cnum , libraries) | { if sess . target . is_like_windows { (dependency_linkage [cnum] != Linkage :: Static) . then_some (libraries) } else { Some (libraries) } }) . flatten () . collect :: < Vec < _ > > () ; native_libraries_from_nonstatics . sort_unstable_by (| a , b | a . name . as_str () . cmp (b . name . as_str ())) ; if sess . target . is_like_windows { for output_path in raw_dylib :: create_raw_dylib_dll_import_libs (sess , archive_builder_builder , native_libraries_from_nonstatics , tmpdir , false ,) { cmd . add_object (& output_path) ; } } else { for link_path in raw_dylib :: create_raw_dylib_elf_stub_shared_objects (sess , native_libraries_from_nonstatics , & raw_dylib_dir ,) { cmd . link_dylib_by_name (& link_path , true , false) ; } } cmd . reset_per_library_state () ; add_late_link_args (cmd , sess , flavor , crate_type , codegen_results) ; add_order_independent_options (cmd , sess , link_output_kind , self_contained_components , flavor , crate_type , codegen_results , out_filename , tmpdir ,) ; add_user_defined_link_args (cmd , sess) ; add_link_script (cmd , sess , tmpdir , crate_type) ; add_post_link_objects (cmd , sess , link_output_kind , self_contained_crt_objects) ; add_post_link_args (cmd , sess , flavor) ; cmd . take_cmd () }
}

macro_rules! add_order_independent_options_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_order_independent_options in module {}", module_path!());
    };
}

mkfn!{
    add_order_independent_options_introspect!();
    fn add_order_independent_options (cmd : & mut dyn Linker , sess : & Session , link_output_kind : LinkOutputKind , self_contained_components : LinkSelfContainedComponents , flavor : LinkerFlavor , crate_type : CrateType , codegen_results : & CodegenResults , out_filename : & Path , tmpdir : & Path ,) { add_lld_args (cmd , sess , flavor , self_contained_components) ; add_apple_link_args (cmd , sess , flavor) ; let apple_sdk_root = add_apple_sdk (cmd , sess , flavor) ; if sess . target . os == "fuchsia" && crate_type == CrateType :: Executable && ! matches ! (flavor , LinkerFlavor :: Gnu (Cc :: Yes , _)) { let prefix = if sess . opts . unstable_opts . sanitizer . contains (SanitizerSet :: ADDRESS) { "asan/" } else { "" } ; cmd . link_arg (format ! ("--dynamic-linker={prefix}ld.so.1")) ; } if sess . target . eh_frame_header { cmd . add_eh_frame_header () ; } cmd . add_no_exec () ; if self_contained_components . is_crt_objects_enabled () { cmd . no_crt_objects () ; } if sess . target . os == "emscripten" { cmd . cc_arg (if sess . opts . unstable_opts . emscripten_wasm_eh { "-fwasm-exceptions" } else if sess . panic_strategy () == PanicStrategy :: Abort { "-sDISABLE_EXCEPTION_CATCHING=1" } else { "-sDISABLE_EXCEPTION_CATCHING=0" }) ; } if flavor == LinkerFlavor :: Llbc { cmd . link_args (& ["--target" , & versioned_llvm_target (sess) , "--target-cpu" , & codegen_results . crate_info . target_cpu ,]) ; if codegen_results . crate_info . target_features . len () > 0 { cmd . link_arg (& format ! ("--target-feature={}" , & codegen_results . crate_info . target_features . join (","))) ; } } else if flavor == LinkerFlavor :: Ptx { cmd . link_args (& ["--fallback-arch" , & codegen_results . crate_info . target_cpu]) ; } else if flavor == LinkerFlavor :: Bpf { cmd . link_args (& ["--cpu" , & codegen_results . crate_info . target_cpu]) ; if let Some (feat) = [sess . opts . cg . target_feature . as_str () , & sess . target . options . features] . into_iter () . find (| feat | ! feat . is_empty ()) { cmd . link_args (& ["--cpu-features" , feat]) ; } } cmd . linker_plugin_lto () ; add_library_search_dirs (cmd , sess , self_contained_components , apple_sdk_root . as_deref ()) ; cmd . output_filename (out_filename) ; if crate_type == CrateType :: Executable && sess . target . is_like_windows && let Some (s) = & codegen_results . crate_info . windows_subsystem { cmd . subsystem (s) ; } if ! sess . link_dead_code () { let keep_metadata = crate_type == CrateType :: Dylib || sess . opts . cg . profile_generate . enabled () ; cmd . gc_sections (keep_metadata) ; } cmd . set_output_kind (link_output_kind , crate_type , out_filename) ; add_relro_args (cmd , sess) ; cmd . optimize () ; let natvis_visualizers = collect_natvis_visualizers (tmpdir , sess , & codegen_results . crate_info . local_crate_name , & codegen_results . crate_info . natvis_debugger_visualizers ,) ; cmd . debuginfo (sess . opts . cg . strip , & natvis_visualizers) ; if ! sess . opts . cg . default_linker_libraries && sess . target . no_default_libraries { cmd . no_default_libraries () ; } if sess . opts . cg . profile_generate . enabled () || sess . instrument_coverage () { cmd . pgo_gen () ; } if sess . opts . cg . control_flow_guard != CFGuard :: Disabled { cmd . control_flow_guard () ; } if sess . opts . unstable_opts . ehcont_guard { cmd . ehcont_guard () ; } add_rpath_args (cmd , sess , codegen_results , out_filename) ; }
}

macro_rules! collect_natvis_visualizers_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_natvis_visualizers in module {}", module_path!());
    };
}

mkfn!{
    collect_natvis_visualizers_introspect!();
    fn collect_natvis_visualizers (tmpdir : & Path , sess : & Session , crate_name : & Symbol , natvis_debugger_visualizers : & BTreeSet < DebuggerVisualizerFile > ,) -> Vec < PathBuf > { let mut visualizer_paths = Vec :: with_capacity (natvis_debugger_visualizers . len ()) ; for (index , visualizer) in natvis_debugger_visualizers . iter () . enumerate () { let visualizer_out_file = tmpdir . join (format ! ("{}-{}.natvis" , crate_name . as_str () , index)) ; match fs :: write (& visualizer_out_file , & visualizer . src) { Ok (()) => { visualizer_paths . push (visualizer_out_file) ; } Err (error) => { sess . dcx () . emit_warn (errors :: UnableToWriteDebuggerVisualizer { path : visualizer_out_file , error , }) ; } } ; } visualizer_paths }
}

macro_rules! add_native_libs_from_crate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_native_libs_from_crate in module {}", module_path!());
    };
}

mkfn!{
    add_native_libs_from_crate_introspect!();
    fn add_native_libs_from_crate (cmd : & mut dyn Linker , sess : & Session , archive_builder_builder : & dyn ArchiveBuilderBuilder , codegen_results : & CodegenResults , tmpdir : & Path , bundled_libs : & FxIndexSet < Symbol > , cnum : CrateNum , link_static : bool , link_dynamic : bool , link_output_kind : LinkOutputKind ,) { if ! sess . opts . unstable_opts . link_native_libraries { return ; } if link_static && cnum != LOCAL_CRATE && ! bundled_libs . is_empty () { let rlib = & codegen_results . crate_info . used_crate_source [& cnum] . rlib . as_ref () . unwrap () . 0 ; archive_builder_builder . extract_bundled_libs (rlib , tmpdir , bundled_libs) . unwrap_or_else (| e | sess . dcx () . emit_fatal (e)) ; } let native_libs = match cnum { LOCAL_CRATE => & codegen_results . crate_info . used_libraries , _ => & codegen_results . crate_info . native_libraries [& cnum] , } ; let mut last = (None , NativeLibKind :: Unspecified , false) ; for lib in native_libs { if ! relevant_lib (sess , lib) { continue ; } last = if (Some (lib . name) , lib . kind , lib . verbatim) == last { continue ; } else { (Some (lib . name) , lib . kind , lib . verbatim) } ; let name = lib . name . as_str () ; let verbatim = lib . verbatim ; match lib . kind { NativeLibKind :: Static { bundle , whole_archive } => { if link_static { let bundle = bundle . unwrap_or (true) ; let whole_archive = whole_archive == Some (true) ; if bundle && cnum != LOCAL_CRATE { if let Some (filename) = lib . filename { let path = tmpdir . join (filename . as_str ()) ; cmd . link_staticlib_by_path (& path , whole_archive) ; } } else { cmd . link_staticlib_by_name (name , verbatim , whole_archive) ; } } } NativeLibKind :: Dylib { as_needed } => { if link_dynamic { cmd . link_dylib_by_name (name , verbatim , as_needed . unwrap_or (true)) } } NativeLibKind :: Unspecified => { if ! link_output_kind . can_link_dylib () && ! sess . target . crt_static_allows_dylibs { if link_static { cmd . link_staticlib_by_name (name , verbatim , false) ; } } else if link_dynamic { cmd . link_dylib_by_name (name , verbatim , true) ; } } NativeLibKind :: Framework { as_needed } => { if link_dynamic { cmd . link_framework_by_name (name , verbatim , as_needed . unwrap_or (true)) } } NativeLibKind :: RawDylib => { } NativeLibKind :: WasmImportModule => { } NativeLibKind :: LinkArg => { if link_static { if verbatim { cmd . verbatim_arg (name) ; } else { cmd . link_arg (name) ; } } } } } }
}

macro_rules! add_local_native_libraries_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_local_native_libraries in module {}", module_path!());
    };
}

mkfn!{
    add_local_native_libraries_introspect!();
    fn add_local_native_libraries (cmd : & mut dyn Linker , sess : & Session , archive_builder_builder : & dyn ArchiveBuilderBuilder , codegen_results : & CodegenResults , tmpdir : & Path , link_output_kind : LinkOutputKind ,) { let link_static = true ; let link_dynamic = true ; add_native_libs_from_crate (cmd , sess , archive_builder_builder , codegen_results , tmpdir , & Default :: default () , LOCAL_CRATE , link_static , link_dynamic , link_output_kind ,) ; }
}

macro_rules! add_upstream_rust_crates_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_upstream_rust_crates in module {}", module_path!());
    };
}

mkfn!{
    add_upstream_rust_crates_introspect!();
    fn add_upstream_rust_crates (cmd : & mut dyn Linker , sess : & Session , archive_builder_builder : & dyn ArchiveBuilderBuilder , codegen_results : & CodegenResults , crate_type : CrateType , tmpdir : & Path , link_output_kind : LinkOutputKind ,) { let data = codegen_results . crate_info . dependency_formats . get (& crate_type) . expect ("failed to find crate type in dependency format list") ; if sess . target . is_like_aix { cmd . link_or_cc_arg ("-bnoipath") ; } for & cnum in & codegen_results . crate_info . used_crates { let linkage = data [cnum] ; let link_static_crate = linkage == Linkage :: Static || (linkage == Linkage :: IncludedFromDylib || linkage == Linkage :: NotLinked) && (codegen_results . crate_info . compiler_builtins == Some (cnum) || codegen_results . crate_info . profiler_runtime == Some (cnum)) ; let mut bundled_libs = Default :: default () ; match linkage { Linkage :: Static | Linkage :: IncludedFromDylib | Linkage :: NotLinked => { if link_static_crate { bundled_libs = codegen_results . crate_info . native_libraries [& cnum] . iter () . filter_map (| lib | lib . filename) . collect () ; add_static_crate (cmd , sess , archive_builder_builder , codegen_results , tmpdir , cnum , & bundled_libs ,) ; } } Linkage :: Dynamic => { let src = & codegen_results . crate_info . used_crate_source [& cnum] ; add_dynamic_crate (cmd , sess , & src . dylib . as_ref () . unwrap () . 0) ; } } let link_static = link_static_crate ; let link_dynamic = false ; add_native_libs_from_crate (cmd , sess , archive_builder_builder , codegen_results , tmpdir , & bundled_libs , cnum , link_static , link_dynamic , link_output_kind ,) ; } }
}

macro_rules! add_upstream_native_libraries_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_upstream_native_libraries in module {}", module_path!());
    };
}

mkfn!{
    add_upstream_native_libraries_introspect!();
    fn add_upstream_native_libraries (cmd : & mut dyn Linker , sess : & Session , archive_builder_builder : & dyn ArchiveBuilderBuilder , codegen_results : & CodegenResults , tmpdir : & Path , link_output_kind : LinkOutputKind ,) { for & cnum in & codegen_results . crate_info . used_crates { let link_static = false ; let link_dynamic = true ; add_native_libs_from_crate (cmd , sess , archive_builder_builder , codegen_results , tmpdir , & Default :: default () , cnum , link_static , link_dynamic , link_output_kind ,) ; } }
}

macro_rules! rehome_sysroot_lib_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rehome_sysroot_lib_dir in module {}", module_path!());
    };
}

mkfn!{
    rehome_sysroot_lib_dir_introspect!();
    fn rehome_sysroot_lib_dir (sess : & Session , lib_dir : & Path) -> PathBuf { let sysroot_lib_path = & sess . target_tlib_path . dir ; let canonical_sysroot_lib_path = { try_canonicalize (sysroot_lib_path) . unwrap_or_else (| _ | sysroot_lib_path . clone ()) } ; let canonical_lib_dir = try_canonicalize (lib_dir) . unwrap_or_else (| _ | lib_dir . to_path_buf ()) ; if canonical_lib_dir == canonical_sysroot_lib_path { sysroot_lib_path . clone () } else { fix_windows_verbatim_for_gcc (lib_dir) } }
}

macro_rules! rehome_lib_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rehome_lib_path in module {}", module_path!());
    };
}

mkfn!{
    rehome_lib_path_introspect!();
    fn rehome_lib_path (sess : & Session , path : & Path) -> PathBuf { if let Some (dir) = path . parent () { let file_name = path . file_name () . expect ("library path has no file name component") ; rehome_sysroot_lib_dir (sess , dir) . join (file_name) } else { fix_windows_verbatim_for_gcc (path) } }
}

macro_rules! add_static_crate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_static_crate in module {}", module_path!());
    };
}

mkfn!{
    add_static_crate_introspect!();
    fn add_static_crate (cmd : & mut dyn Linker , sess : & Session , archive_builder_builder : & dyn ArchiveBuilderBuilder , codegen_results : & CodegenResults , tmpdir : & Path , cnum : CrateNum , bundled_lib_file_names : & FxIndexSet < Symbol > ,) { let src = & codegen_results . crate_info . used_crate_source [& cnum] ; let cratepath = & src . rlib . as_ref () . unwrap () . 0 ; let mut link_upstream = | path : & Path | cmd . link_staticlib_by_path (& rehome_lib_path (sess , path) , false) ; if ! are_upstream_rust_objects_already_included (sess) || ignored_for_lto (sess , & codegen_results . crate_info , cnum) { link_upstream (cratepath) ; return ; } let dst = tmpdir . join (cratepath . file_name () . unwrap ()) ; let name = cratepath . file_name () . unwrap () . to_str () . unwrap () ; let name = & name [3 .. name . len () - 5] ; let bundled_lib_file_names = bundled_lib_file_names . clone () ; sess . prof . generic_activity_with_arg ("link_altering_rlib" , name) . run (| | { let canonical_name = name . replace ('-' , "_") ; let upstream_rust_objects_already_included = are_upstream_rust_objects_already_included (sess) ; let is_builtins = sess . target . no_builtins || ! codegen_results . crate_info . is_no_builtins . contains (& cnum) ; let mut archive = archive_builder_builder . new_archive_builder (sess) ; if let Err (error) = archive . add_archive (cratepath , Box :: new (move | f | { if f == METADATA_FILENAME { return true ; } let canonical = f . replace ('-' , "_") ; let is_rust_object = canonical . starts_with (& canonical_name) && looks_like_rust_object_file (f) ; if upstream_rust_objects_already_included && is_rust_object && is_builtins { return true ; } if bundled_lib_file_names . contains (& Symbol :: intern (f)) { return true ; } false }) ,) { sess . dcx () . emit_fatal (errors :: RlibArchiveBuildFailure { path : cratepath . clone () , error }) ; } if archive . build (& dst) { link_upstream (& dst) ; } }) ; }
}

macro_rules! add_dynamic_crate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_dynamic_crate in module {}", module_path!());
    };
}

mkfn!{
    add_dynamic_crate_introspect!();
    fn add_dynamic_crate (cmd : & mut dyn Linker , sess : & Session , cratepath : & Path) { cmd . link_dylib_by_path (& rehome_lib_path (sess , cratepath) , true) ; }
}

macro_rules! relevant_lib_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function relevant_lib in module {}", module_path!());
    };
}

mkfn!{
    relevant_lib_introspect!();
    fn relevant_lib (sess : & Session , lib : & NativeLib) -> bool { match lib . cfg { Some (ref cfg) => { eval_config_entry (sess , cfg , CRATE_NODE_ID , None , ShouldEmit :: ErrorsAndLints) . as_bool () } None => true , } }
}

macro_rules! are_upstream_rust_objects_already_included_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function are_upstream_rust_objects_already_included in module {}", module_path!());
    };
}

mkfn!{
    are_upstream_rust_objects_already_included_introspect!();
    pub (crate) fn are_upstream_rust_objects_already_included (sess : & Session) -> bool { match sess . lto () { config :: Lto :: Fat => true , config :: Lto :: Thin => { ! sess . opts . cg . linker_plugin_lto . enabled () } config :: Lto :: No | config :: Lto :: ThinLocal => false , } }
}

macro_rules! add_apple_link_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_apple_link_args in module {}", module_path!());
    };
}

mkfn!{
    add_apple_link_args_introspect!();
    # [doc = " We need to communicate five things to the linker on Apple/Darwin targets:"] # [doc = " - The architecture."] # [doc = " - The operating system (and that it's an Apple platform)."] # [doc = " - The environment."] # [doc = " - The deployment target."] # [doc = " - The SDK version."] fn add_apple_link_args (cmd : & mut dyn Linker , sess : & Session , flavor : LinkerFlavor) { if ! sess . target . is_like_darwin { return ; } let LinkerFlavor :: Darwin (cc , _) = flavor else { return ; } ; let llvm_arch = sess . target . llvm_target . split_once ('-') . expect ("LLVM target must have arch") . 0 ; let target_os = & * sess . target . os ; let target_env = & * sess . target . env ; let ld64_arch = match llvm_arch { "armv7k" => "armv7k" , "armv7s" => "armv7s" , "arm64" => "arm64" , "arm64e" => "arm64e" , "arm64_32" => "arm64_32" , "i386" | "i686" => "i386" , "x86_64" => "x86_64" , "x86_64h" => "x86_64h" , _ => bug ! ("unsupported architecture in Apple target: {}" , sess . target . llvm_target) , } ; if cc == Cc :: No { cmd . link_args (& ["-arch" , ld64_arch]) ; let platform_name = match (target_os , target_env) { (os , "") => os , ("ios" , "macabi") => "mac-catalyst" , ("ios" , "sim") => "ios-simulator" , ("tvos" , "sim") => "tvos-simulator" , ("watchos" , "sim") => "watchos-simulator" , ("visionos" , "sim") => "visionos-simulator" , _ => bug ! ("invalid OS/env combination for Apple target: {target_os}, {target_env}") , } ; let min_version = sess . apple_deployment_target () . fmt_full () . to_string () ; let sdk_version = & * min_version ; cmd . link_args (& ["-platform_version" , platform_name , & * min_version , sdk_version]) ; } else { if target_os == "macos" { cmd . cc_args (& ["-arch" , ld64_arch]) ; let version = sess . apple_deployment_target () . fmt_full () ; cmd . cc_arg (& format ! ("-mmacosx-version-min={version}")) ; } else { cmd . cc_args (& ["-target" , & versioned_llvm_target (sess)]) ; } } }
}

macro_rules! add_apple_sdk_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_apple_sdk in module {}", module_path!());
    };
}

mkfn!{
    add_apple_sdk_introspect!();
    fn add_apple_sdk (cmd : & mut dyn Linker , sess : & Session , flavor : LinkerFlavor) -> Option < PathBuf > { if ! sess . target . is_like_darwin { return None ; } let LinkerFlavor :: Darwin (cc , _) = flavor else { return None ; } ; let sdkroot = sess . time ("get_apple_sdk_root" , | | get_apple_sdk_root (sess)) ? ; if cc == Cc :: Yes { cmd . cmd () . env ("SDKROOT" , & sdkroot) ; } else { cmd . link_arg ("-syslibroot") ; cmd . link_arg (& sdkroot) ; } Some (sdkroot) }
}

macro_rules! get_apple_sdk_root_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_apple_sdk_root in module {}", module_path!());
    };
}

mkfn!{
    get_apple_sdk_root_introspect!();
    fn get_apple_sdk_root (sess : & Session) -> Option < PathBuf > { if let Ok (sdkroot) = env :: var ("SDKROOT") { let p = PathBuf :: from (& sdkroot) ; match & * apple :: sdk_name (& sess . target) . to_lowercase () { "appletvos" if sdkroot . contains ("TVSimulator.platform") || sdkroot . contains ("MacOSX.platform") => { } "appletvsimulator" if sdkroot . contains ("TVOS.platform") || sdkroot . contains ("MacOSX.platform") => { } "iphoneos" if sdkroot . contains ("iPhoneSimulator.platform") || sdkroot . contains ("MacOSX.platform") => { } "iphonesimulator" if sdkroot . contains ("iPhoneOS.platform") || sdkroot . contains ("MacOSX.platform") => { } "macosx" if sdkroot . contains ("iPhoneOS.platform") || sdkroot . contains ("iPhoneSimulator.platform") || sdkroot . contains ("AppleTVOS.platform") || sdkroot . contains ("AppleTVSimulator.platform") || sdkroot . contains ("WatchOS.platform") || sdkroot . contains ("WatchSimulator.platform") || sdkroot . contains ("XROS.platform") || sdkroot . contains ("XRSimulator.platform") => { } "watchos" if sdkroot . contains ("WatchSimulator.platform") || sdkroot . contains ("MacOSX.platform") => { } "watchsimulator" if sdkroot . contains ("WatchOS.platform") || sdkroot . contains ("MacOSX.platform") => { } "xros" if sdkroot . contains ("XRSimulator.platform") || sdkroot . contains ("MacOSX.platform") => { } "xrsimulator" if sdkroot . contains ("XROS.platform") || sdkroot . contains ("MacOSX.platform") => { } _ if ! p . is_absolute () || p == Path :: new ("/") || ! p . exists () => { } _ => return Some (p) , } } apple :: get_sdk_root (sess) }
}

macro_rules! add_lld_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_lld_args in module {}", module_path!());
    };
}

mkfn!{
    add_lld_args_introspect!();
    # [doc = " When using the linker flavors opting in to `lld`, add the necessary paths and arguments to"] # [doc = " invoke it:"] # [doc = " - when the self-contained linker flag is active: the build of `lld` distributed with rustc,"] # [doc = " - or any `lld` available to `cc`."] fn add_lld_args (cmd : & mut dyn Linker , sess : & Session , flavor : LinkerFlavor , self_contained_components : LinkSelfContainedComponents ,) { debug ! ("add_lld_args requested, flavor: '{:?}', target self-contained components: {:?}" , flavor , self_contained_components ,) ; if ! (flavor . uses_cc () && flavor . uses_lld ()) { return ; } let self_contained_cli = sess . opts . cg . link_self_contained . is_linker_enabled () ; let self_contained_target = self_contained_components . is_linker_enabled () ; let self_contained_linker = self_contained_cli || self_contained_target ; if self_contained_linker && ! sess . opts . cg . link_self_contained . is_linker_disabled () { let mut linker_path_exists = false ; for path in sess . get_tools_search_paths (false) { let linker_path = path . join ("gcc-ld") ; linker_path_exists |= linker_path . exists () ; cmd . cc_arg ({ let mut arg = OsString :: from ("-B") ; arg . push (linker_path) ; arg }) ; } if ! linker_path_exists { sess . dcx () . emit_fatal (errors :: SelfContainedLinkerMissing) ; } } if ! sess . target . is_like_wasm { cmd . cc_arg ("-fuse-ld=lld") ; } if ! flavor . is_gnu () { if sess . target . linker_flavor != sess . host . linker_flavor { cmd . cc_arg (format ! ("--target={}" , versioned_llvm_target (sess))) ; } } }
}

macro_rules! warn_if_linked_with_gold_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function warn_if_linked_with_gold in module {}", module_path!());
    };
}

mkfn!{
    warn_if_linked_with_gold_introspect!();
    fn warn_if_linked_with_gold (sess : & Session , path : & Path) -> Result < () , Box < dyn std :: error :: Error > > { use object :: read :: elf :: { FileHeader , SectionHeader } ; use object :: read :: { ReadCache , ReadRef , Result } ; use object :: { Endianness , elf } ; fn elf_has_gold_version_note < 'a > (elf : & impl FileHeader , data : impl ReadRef < 'a > ,) -> Result < bool > { let endian = elf . endian () ? ; let section = elf . sections (endian , data) ? . section_by_name (endian , b".note.gnu.gold-version") ; if let Some ((_ , section)) = section && let Some (mut notes) = section . notes (endian , data) ? { return Ok (notes . any (| note | { note . is_ok_and (| note | note . n_type (endian) == elf :: NT_GNU_GOLD_VERSION) })) ; } Ok (false) } let data = ReadCache :: new (BufReader :: new (File :: open (path) ?)) ; let was_linked_with_gold = if sess . target . pointer_width == 64 { let elf = elf :: FileHeader64 :: < Endianness > :: parse (& data) ? ; elf_has_gold_version_note (elf , & data) ? } else if sess . target . pointer_width == 32 { let elf = elf :: FileHeader32 :: < Endianness > :: parse (& data) ? ; elf_has_gold_version_note (elf , & data) ? } else { return Ok (()) ; } ; if was_linked_with_gold { let mut warn = sess . dcx () . struct_warn ("the gold linker is deprecated and has known bugs with Rust") ; warn . help ("consider using LLD or ld from GNU binutils instead") ; warn . emit () ; } Ok (()) }
}