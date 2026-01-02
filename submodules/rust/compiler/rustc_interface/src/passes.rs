mkuse!{use std :: any :: Any ;}
mkuse!{use std :: ffi :: { OsStr , OsString } ;}
mkuse!{use std :: io :: { self , BufWriter , Write } ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use std :: sync :: { Arc , LazyLock , OnceLock } ;}
mkuse!{use std :: { env , fs , iter } ;}
mkuse!{use rustc_ast as ast ;}
mkuse!{use rustc_attr_parsing :: { AttributeParser , ShouldEmit } ;}
mkuse!{use rustc_codegen_ssa :: traits :: CodegenBackend ;}
mkuse!{use rustc_data_structures :: jobserver :: Proxy ;}
mkuse!{use rustc_data_structures :: steal :: Steal ;}
mkuse!{use rustc_data_structures :: sync :: { AppendOnlyIndexVec , FreezeLock , WorkerLocal } ;}
mkuse!{use rustc_data_structures :: { parallel , thousands } ;}
mkuse!{use rustc_errors :: timings :: TimingSection ;}
mkuse!{use rustc_expand :: base :: { ExtCtxt , LintStoreExpand } ;}
mkuse!{use rustc_feature :: Features ;}
mkuse!{use rustc_fs_util :: try_canonicalize ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_hir :: def_id :: { LOCAL_CRATE , StableCrateId , StableCrateIdMap } ;}
mkuse!{use rustc_hir :: definitions :: Definitions ;}
mkuse!{use rustc_hir :: limit :: Limit ;}
mkuse!{use rustc_incremental :: setup_dep_graph ;}
mkuse!{use rustc_lint :: { BufferedEarlyLint , EarlyCheckNode , LintStore , unerased_lint_store } ;}
mkuse!{use rustc_metadata :: EncodedMetadata ;}
mkuse!{use rustc_metadata :: creader :: CStore ;}
mkuse!{use rustc_middle :: arena :: Arena ;}
mkuse!{use rustc_middle :: dep_graph :: DepsType ;}
mkuse!{use rustc_middle :: ty :: { self , CurrentGcx , GlobalCtxt , RegisteredTools , TyCtxt } ;}
mkuse!{use rustc_middle :: util :: Providers ;}
mkuse!{use rustc_parse :: lexer :: StripTokens ;}
mkuse!{use rustc_parse :: { new_parser_from_file , new_parser_from_source_str , unwrap_or_emit_fatal } ;}
mkuse!{use rustc_passes :: { abi_test , input_stats , layout_test } ;}
mkuse!{use rustc_resolve :: { Resolver , ResolverOutputs } ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: config :: { CrateType , Input , OutFileName , OutputFilenames , OutputType } ;}
mkuse!{use rustc_session :: cstore :: Untracked ;}
mkuse!{use rustc_session :: output :: { collect_crate_types , filename_for_input } ;}
mkuse!{use rustc_session :: parse :: feature_err ;}
mkuse!{use rustc_session :: search_paths :: PathKind ;}
mkuse!{use rustc_span :: { DUMMY_SP , ErrorGuaranteed , ExpnKind , FileName , SourceFileHash , SourceFileHashAlgorithm , Span , Symbol , sym , } ;}
mkuse!{use rustc_target :: spec :: PanicStrategy ;}
mkuse!{use rustc_trait_selection :: { solve , traits } ;}
mkuse!{use tracing :: { info , instrument } ;}
mkuse!{use crate :: interface :: Compiler ;}
mkuse!{use crate :: { errors , limits , proc_macro_decls , util } ;}

macro_rules! parse_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse in module {}", module_path!());
    };
}

mkfn!{
    parse_introspect!();
    pub fn parse < 'a > (sess : & 'a Session) -> ast :: Crate { let mut krate = sess . time ("parse_crate" , | | { let mut parser = unwrap_or_emit_fatal (match & sess . io . input { Input :: File (file) => new_parser_from_file (& sess . psess , file , StripTokens :: ShebangAndFrontmatter , None ,) , Input :: Str { input , name } => new_parser_from_source_str (& sess . psess , name . clone () , input . clone () , StripTokens :: ShebangAndFrontmatter ,) , }) ; parser . parse_crate_mod () }) . unwrap_or_else (| parse_error | { let guar : ErrorGuaranteed = parse_error . emit () ; guar . raise_fatal () ; }) ; rustc_builtin_macros :: cmdline_attrs :: inject (& mut krate , & sess . psess , & sess . opts . unstable_opts . crate_attr ,) ; krate }
}

macro_rules! pre_expansion_lint_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pre_expansion_lint in module {}", module_path!());
    };
}

mkfn!{
    pre_expansion_lint_introspect!();
    fn pre_expansion_lint < 'a > (sess : & Session , features : & Features , lint_store : & LintStore , registered_tools : & RegisteredTools , check_node : impl EarlyCheckNode < 'a > , node_name : Symbol ,) { sess . prof . generic_activity_with_arg ("pre_AST_expansion_lint_checks" , node_name . as_str ()) . run (| | { rustc_lint :: check_ast_node (sess , None , features , true , lint_store , registered_tools , None , rustc_lint :: BuiltinCombinedPreExpansionLintPass :: new () , check_node ,) ; } ,) ; }
}
mkitem!{mkstruct!{struct LintStoreExpandImpl < 'a > (& 'a LintStore) ;}}
mkitem!{mkimpl!{impl LintStoreExpand for LintStoreExpandImpl < '_ > { fn pre_expansion_lint (& self , sess : & Session , features : & Features , registered_tools : & RegisteredTools , node_id : ast :: NodeId , attrs : & [ast :: Attribute] , items : & [Box < ast :: Item >] , name : Symbol ,) { pre_expansion_lint (sess , features , self . 0 , registered_tools , (node_id , attrs , items) , name) ; } }}}

macro_rules! configure_and_expand_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function configure_and_expand in module {}", module_path!());
    };
}

mkfn!{
    configure_and_expand_introspect!();
    # [doc = " Runs the \"early phases\" of the compiler: initial `cfg` processing,"] # [doc = " syntax expansion, secondary `cfg` expansion, synthesis of a test"] # [doc = " harness if one is to be provided, injection of a dependency on the"] # [doc = " standard library and prelude, and name resolution."] # [instrument (level = "trace" , skip (krate , resolver))] fn configure_and_expand (mut krate : ast :: Crate , pre_configured_attrs : & [ast :: Attribute] , resolver : & mut Resolver < '_ , '_ > ,) -> ast :: Crate { let tcx = resolver . tcx () ; let sess = tcx . sess ; let features = tcx . features () ; let lint_store = unerased_lint_store (tcx . sess) ; let crate_name = tcx . crate_name (LOCAL_CRATE) ; let lint_check_node = (& krate , pre_configured_attrs) ; pre_expansion_lint (sess , features , lint_store , tcx . registered_tools (()) , lint_check_node , crate_name ,) ; rustc_builtin_macros :: register_builtin_macros (resolver) ; let num_standard_library_imports = sess . time ("crate_injection" , | | { rustc_builtin_macros :: standard_library_imports :: inject (& mut krate , pre_configured_attrs , resolver , sess , features ,) }) ; util :: check_attr_crate_type (sess , pre_configured_attrs , resolver . lint_buffer ()) ; krate = sess . time ("macro_expand_crate" , | | { let mut old_path = OsString :: new () ; if cfg ! (windows) { old_path = env :: var_os ("PATH") . unwrap_or (old_path) ; let mut new_path = Vec :: from_iter (sess . host_filesearch () . search_paths (PathKind :: All) . map (| p | p . dir . clone ()) ,) ; for path in env :: split_paths (& old_path) { if ! new_path . contains (& path) { new_path . push (path) ; } } unsafe { env :: set_var ("PATH" , & env :: join_paths (new_path . iter () . filter (| p | env :: join_paths (iter :: once (p)) . is_ok ()) ,) . unwrap () ,) ; } } let recursion_limit = get_recursion_limit (pre_configured_attrs , sess) ; let cfg = rustc_expand :: expand :: ExpansionConfig { crate_name , features , recursion_limit , trace_mac : sess . opts . unstable_opts . trace_macros , should_test : sess . is_test_crate () , span_debug : sess . opts . unstable_opts . span_debug , proc_macro_backtrace : sess . opts . unstable_opts . proc_macro_backtrace , } ; let lint_store = LintStoreExpandImpl (lint_store) ; let mut ecx = ExtCtxt :: new (sess , cfg , resolver , Some (& lint_store)) ; ecx . num_standard_library_imports = num_standard_library_imports ; let krate = sess . time ("expand_crate" , | | ecx . monotonic_expander () . expand_crate (krate)) ; if ecx . nb_macro_errors > 0 { sess . dcx () . abort_if_errors () ; } sess . psess . buffered_lints . with_lock (| buffered_lints : & mut Vec < BufferedEarlyLint > | { buffered_lints . append (& mut ecx . buffered_early_lint) ; }) ; sess . time ("check_unused_macros" , | | { ecx . check_unused_macros () ; }) ; if ecx . reduced_recursion_limit . is_some () { sess . dcx () . abort_if_errors () ; unreachable ! () ; } if cfg ! (windows) { unsafe { env :: set_var ("PATH" , & old_path) ; } } if ecx . sess . opts . unstable_opts . macro_stats { print_macro_stats (& ecx) ; } krate }) ; sess . time ("maybe_building_test_harness" , | | { rustc_builtin_macros :: test_harness :: inject (& mut krate , sess , features , resolver) }) ; let has_proc_macro_decls = sess . time ("AST_validation" , | | { rustc_ast_passes :: ast_validation :: check_crate (sess , features , & krate , tcx . is_sdylib_interface_build () , resolver . lint_buffer () ,) }) ; let crate_types = tcx . crate_types () ; let is_executable_crate = crate_types . contains (& CrateType :: Executable) ; let is_proc_macro_crate = crate_types . contains (& CrateType :: ProcMacro) ; if crate_types . len () > 1 { if is_executable_crate { sess . dcx () . emit_err (errors :: MixedBinCrate) ; } if is_proc_macro_crate { sess . dcx () . emit_err (errors :: MixedProcMacroCrate) ; } } if crate_types . contains (& CrateType :: Sdylib) && ! tcx . features () . export_stable () { feature_err (sess , sym :: export_stable , DUMMY_SP , "`sdylib` crate type is unstable") . emit () ; } if is_proc_macro_crate && sess . panic_strategy () == PanicStrategy :: Abort { sess . dcx () . emit_warn (errors :: ProcMacroCratePanicAbort) ; } sess . time ("maybe_create_a_macro_crate" , | | { let is_test_crate = sess . is_test_crate () ; rustc_builtin_macros :: proc_macro_harness :: inject (& mut krate , sess , features , resolver , is_proc_macro_crate , has_proc_macro_decls , is_test_crate , sess . dcx () ,) }) ; resolver . resolve_crate (& krate) ; CStore :: from_tcx (tcx) . report_incompatible_target_modifiers (tcx , & krate) ; CStore :: from_tcx (tcx) . report_incompatible_async_drop_feature (tcx , & krate) ; krate }
}

macro_rules! print_macro_stats_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function print_macro_stats in module {}", module_path!());
    };
}

mkfn!{
    print_macro_stats_introspect!();
    fn print_macro_stats (ecx : & ExtCtxt < '_ >) { use std :: fmt :: Write ; let crate_name = ecx . ecfg . crate_name . as_str () ; let crate_name = if crate_name == "build_script_build" { let pkg_name = std :: env :: var ("CARGO_PKG_NAME") . unwrap_or_else (| _ | "<unknown crate>" . to_string ()) ; format ! ("{pkg_name} build script") } else { crate_name . to_string () } ; # [allow (rustc :: potential_query_instability)] let mut macro_stats : Vec < _ > = ecx . macro_stats . iter () . map (| ((name , kind) , stat) | { (stat . bytes , stat . lines , stat . uses , name , * kind) }) . collect () ; macro_stats . sort_unstable () ; macro_stats . reverse () ; let prefix = "macro-stats" ; let name_w = 32 ; let uses_w = 7 ; let lines_w = 11 ; let avg_lines_w = 11 ; let bytes_w = 11 ; let avg_bytes_w = 11 ; let banner_w = name_w + uses_w + lines_w + avg_lines_w + bytes_w + avg_bytes_w ; let mut s = String :: new () ; _ = writeln ! (s , "{prefix} {}" , "=" . repeat (banner_w)) ; _ = writeln ! (s , "{prefix} MACRO EXPANSION STATS: {}" , crate_name) ; _ = writeln ! (s , "{prefix} {:<name_w$}{:>uses_w$}{:>lines_w$}{:>avg_lines_w$}{:>bytes_w$}{:>avg_bytes_w$}" , "Macro Name" , "Uses" , "Lines" , "Avg Lines" , "Bytes" , "Avg Bytes" ,) ; _ = writeln ! (s , "{prefix} {}" , "-" . repeat (banner_w)) ; if macro_stats . is_empty () { _ = writeln ! (s , "{prefix} (none)") ; } for (bytes , lines , uses , name , kind) in macro_stats { let mut name = ExpnKind :: Macro (kind , * name) . descr () ; let uses_with_underscores = thousands :: usize_with_underscores (uses) ; let avg_lines = lines as f64 / uses as f64 ; let avg_bytes = bytes as f64 / uses as f64 ; let mut uses_w = uses_w ; if name . len () + uses_with_underscores . len () >= name_w + uses_w { _ = writeln ! (s , "{prefix} {:<name_w$}" , name) ; name = String :: new () ; } else if name . len () >= name_w { uses_w -= name . len () - name_w ; } ; _ = writeln ! (s , "{prefix} {:<name_w$}{:>uses_w$}{:>lines_w$}{:>avg_lines_w$}{:>bytes_w$}{:>avg_bytes_w$}" , name , uses_with_underscores , thousands :: usize_with_underscores (lines) , thousands :: f64p1_with_underscores (avg_lines) , thousands :: usize_with_underscores (bytes) , thousands :: f64p1_with_underscores (avg_bytes) ,) ; } _ = writeln ! (s , "{prefix} {}" , "=" . repeat (banner_w)) ; eprint ! ("{s}") ; }
}

macro_rules! early_lint_checks_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function early_lint_checks in module {}", module_path!());
    };
}

mkfn!{
    early_lint_checks_introspect!();
    fn early_lint_checks (tcx : TyCtxt < '_ > , () : ()) { let sess = tcx . sess ; let (resolver , krate) = & * tcx . resolver_for_lowering () . borrow () ; let mut lint_buffer = resolver . lint_buffer . steal () ; if sess . opts . unstable_opts . input_stats { input_stats :: print_ast_stats (tcx , krate) ; } sess . time ("complete_gated_feature_checking" , | | { rustc_ast_passes :: feature_gate :: check_crate (krate , sess , tcx . features ()) ; }) ; sess . psess . buffered_lints . with_lock (| buffered_lints | { info ! ("{} parse sess buffered_lints" , buffered_lints . len ()) ; for early_lint in buffered_lints . drain (..) { lint_buffer . add_early_lint (early_lint) ; } }) ; sess . psess . bad_unicode_identifiers . with_lock (| identifiers | { for (ident , mut spans) in identifiers . drain (..) { spans . sort () ; if ident == sym :: ferris { enum FerrisFix { SnakeCase , ScreamingSnakeCase , PascalCase , } impl FerrisFix { const fn as_str (self) -> & 'static str { match self { FerrisFix :: SnakeCase => "ferris" , FerrisFix :: ScreamingSnakeCase => "FERRIS" , FerrisFix :: PascalCase => "Ferris" , } } } let first_span = spans [0] ; let prev_source = sess . psess . source_map () . span_to_prev_source (first_span) ; let ferris_fix = prev_source . map_or (FerrisFix :: SnakeCase , | source | { let mut source_before_ferris = source . trim_end () . split_whitespace () . rev () ; match source_before_ferris . next () { Some ("struct" | "trait" | "mod" | "union" | "type" | "enum") => { FerrisFix :: PascalCase } Some ("const" | "static") => FerrisFix :: ScreamingSnakeCase , Some ("mut") if source_before_ferris . next () == Some ("static") => { FerrisFix :: ScreamingSnakeCase } _ => FerrisFix :: SnakeCase , } }) . as_str () ; sess . dcx () . emit_err (errors :: FerrisIdentifier { spans , first_span , ferris_fix }) ; } else { sess . dcx () . emit_err (errors :: EmojiIdentifier { spans , ident }) ; } } }) ; let lint_store = unerased_lint_store (tcx . sess) ; rustc_lint :: check_ast_node (sess , Some (tcx) , tcx . features () , false , lint_store , tcx . registered_tools (()) , Some (lint_buffer) , rustc_lint :: BuiltinCombinedEarlyLintPass :: new () , (& * * krate , & * krate . attrs) ,) }
}

macro_rules! env_var_os_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function env_var_os in module {}", module_path!());
    };
}

mkfn!{
    env_var_os_introspect!();
    fn env_var_os < 'tcx > (tcx : TyCtxt < 'tcx > , key : & 'tcx OsStr) -> Option < & 'tcx OsStr > { let value = env :: var_os (key) ; let value_tcx = value . as_ref () . map (| value | { let encoded_bytes = tcx . arena . alloc_slice (value . as_encoded_bytes ()) ; debug_assert_eq ! (value . as_encoded_bytes () , encoded_bytes) ; unsafe { OsStr :: from_encoded_bytes_unchecked (encoded_bytes) } }) ; tcx . sess . psess . env_depinfo . borrow_mut () . insert ((Symbol :: intern (& key . to_string_lossy ()) , value . as_ref () . and_then (| value | value . to_str ()) . map (| value | Symbol :: intern (& value)) ,)) ; value_tcx }
}

macro_rules! generated_output_paths_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function generated_output_paths in module {}", module_path!());
    };
}

mkfn!{
    generated_output_paths_introspect!();
    fn generated_output_paths (tcx : TyCtxt < '_ > , outputs : & OutputFilenames , exact_name : bool , crate_name : Symbol ,) -> Vec < PathBuf > { let sess = tcx . sess ; let mut out_filenames = Vec :: new () ; for output_type in sess . opts . output_types . keys () { let out_filename = outputs . path (* output_type) ; let file = out_filename . as_path () . to_path_buf () ; match * output_type { OutputType :: Exe if ! exact_name => { for crate_type in tcx . crate_types () . iter () { let p = filename_for_input (sess , * crate_type , crate_name , outputs) ; out_filenames . push (p . as_path () . to_path_buf ()) ; } } OutputType :: DepInfo if sess . opts . unstable_opts . dep_info_omit_d_target => { } OutputType :: DepInfo if out_filename . is_stdout () => { } _ => { out_filenames . push (file) ; } } } out_filenames }
}

macro_rules! output_contains_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function output_contains_path in module {}", module_path!());
    };
}

mkfn!{
    output_contains_path_introspect!();
    fn output_contains_path (output_paths : & [PathBuf] , input_path : & Path) -> bool { let input_path = try_canonicalize (input_path) . ok () ; if input_path . is_none () { return false ; } output_paths . iter () . any (| output_path | try_canonicalize (output_path) . ok () == input_path) }
}

macro_rules! output_conflicts_with_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function output_conflicts_with_dir in module {}", module_path!());
    };
}

mkfn!{
    output_conflicts_with_dir_introspect!();
    fn output_conflicts_with_dir (output_paths : & [PathBuf]) -> Option < & PathBuf > { output_paths . iter () . find (| output_path | output_path . is_dir ()) }
}

macro_rules! escape_dep_filename_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function escape_dep_filename in module {}", module_path!());
    };
}

mkfn!{
    escape_dep_filename_introspect!();
    fn escape_dep_filename (filename : & str) -> String { filename . replace (' ' , "\\ ") }
}

macro_rules! escape_dep_env_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function escape_dep_env in module {}", module_path!());
    };
}

mkfn!{
    escape_dep_env_introspect!();
    fn escape_dep_env (symbol : Symbol) -> String { let s = symbol . as_str () ; let mut escaped = String :: with_capacity (s . len ()) ; for c in s . chars () { match c { '\n' => escaped . push_str (r"\n") , '\r' => escaped . push_str (r"\r") , '\\' => escaped . push_str (r"\\") , _ => escaped . push (c) , } } escaped }
}

macro_rules! write_out_deps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_out_deps in module {}", module_path!());
    };
}

mkfn!{
    write_out_deps_introspect!();
    fn write_out_deps (tcx : TyCtxt < '_ > , outputs : & OutputFilenames , out_filenames : & [PathBuf]) { let sess = tcx . sess ; if ! sess . opts . output_types . contains_key (& OutputType :: DepInfo) { return ; } let deps_output = outputs . path (OutputType :: DepInfo) ; let deps_filename = deps_output . as_path () ; let result : io :: Result < () > = try { let mut files : Vec < (String , u64 , Option < SourceFileHash >) > = sess . source_map () . files () . iter () . filter (| fmap | fmap . is_real_file ()) . filter (| fmap | ! fmap . is_imported ()) . map (| fmap | { (escape_dep_filename (& fmap . name . prefer_local () . to_string ()) , fmap . source_len . 0 as u64 , fmap . checksum_hash ,) }) . collect () ; let checksum_hash_algo = sess . opts . unstable_opts . checksum_hash_algorithm ; let file_depinfo = sess . psess . file_depinfo . borrow () ; let normalize_path = | path : PathBuf | { let file = FileName :: from (path) ; escape_dep_filename (& file . prefer_local () . to_string ()) } ; fn hash_iter_files < P : AsRef < Path > > (it : impl Iterator < Item = P > , checksum_hash_algo : Option < SourceFileHashAlgorithm > ,) -> impl Iterator < Item = (P , u64 , Option < SourceFileHash >) > { it . map (move | path | { match checksum_hash_algo . and_then (| algo | { fs :: File :: open (path . as_ref ()) . and_then (| mut file | { SourceFileHash :: new (algo , & mut file) . map (| h | (file , h)) }) . and_then (| (file , h) | file . metadata () . map (| m | (m . len () , h))) . map_err (| e | { tracing :: error ! ("failed to compute checksum, omitting it from dep-info {} {e}" , path . as_ref () . display ()) }) . ok () }) { Some ((file_len , checksum)) => (path , file_len , Some (checksum)) , None => (path , 0 , None) , } }) } let extra_tracked_files = hash_iter_files (file_depinfo . iter () . map (| path_sym | normalize_path (PathBuf :: from (path_sym . as_str ()))) , checksum_hash_algo ,) ; files . extend (extra_tracked_files) ; if let Some (ref profile_instr) = sess . opts . cg . profile_use { files . extend (hash_iter_files (iter :: once (normalize_path (profile_instr . as_path () . to_path_buf ())) , checksum_hash_algo ,)) ; } if let Some (ref profile_sample) = sess . opts . unstable_opts . profile_sample_use { files . extend (hash_iter_files (iter :: once (normalize_path (profile_sample . as_path () . to_path_buf ())) , checksum_hash_algo ,)) ; } for debugger_visualizer in tcx . debugger_visualizers (LOCAL_CRATE) { files . extend (hash_iter_files (iter :: once (normalize_path (debugger_visualizer . path . clone () . unwrap ())) , checksum_hash_algo ,)) ; } if sess . binary_dep_depinfo () { if let Some (ref backend) = sess . opts . unstable_opts . codegen_backend { if backend . contains ('.') { files . extend (hash_iter_files (iter :: once (backend . to_string ()) , checksum_hash_algo ,)) ; } } for & cnum in tcx . crates (()) { let source = tcx . used_crate_source (cnum) ; if let Some ((path , _)) = & source . dylib { files . extend (hash_iter_files (iter :: once (escape_dep_filename (& path . display () . to_string ())) , checksum_hash_algo ,)) ; } if let Some ((path , _)) = & source . rlib { files . extend (hash_iter_files (iter :: once (escape_dep_filename (& path . display () . to_string ())) , checksum_hash_algo ,)) ; } if let Some ((path , _)) = & source . rmeta { files . extend (hash_iter_files (iter :: once (escape_dep_filename (& path . display () . to_string ())) , checksum_hash_algo ,)) ; } } } let write_deps_to_file = | file : & mut dyn Write | -> io :: Result < () > { for path in out_filenames { writeln ! (file , "{}: {}\n" , path . display () , files . iter () . map (| (path , _file_len , _checksum_hash_algo) | path . as_str ()) . intersperse (" ") . collect ::< String > ()) ? ; } for (path , _file_len , _checksum_hash_algo) in & files { writeln ! (file , "{path}:") ? ; } let env_depinfo = sess . psess . env_depinfo . borrow () ; if ! env_depinfo . is_empty () { # [allow (rustc :: potential_query_instability)] let mut envs : Vec < _ > = env_depinfo . iter () . map (| (k , v) | (escape_dep_env (* k) , v . map (escape_dep_env))) . collect () ; envs . sort_unstable () ; writeln ! (file) ? ; for (k , v) in envs { write ! (file , "# env-dep:{k}") ? ; if let Some (v) = v { write ! (file , "={v}") ? ; } writeln ! (file) ? ; } } if sess . opts . unstable_opts . checksum_hash_algorithm () . is_some () { files . iter () . filter_map (| (path , file_len , hash_algo) | { hash_algo . map (| hash_algo | (path , file_len , hash_algo)) }) . try_for_each (| (path , file_len , checksum_hash) | { writeln ! (file , "# checksum:{checksum_hash} file_len:{file_len} {path}") }) ? ; } Ok (()) } ; match deps_output { OutFileName :: Stdout => { let mut file = BufWriter :: new (io :: stdout ()) ; write_deps_to_file (& mut file) ? ; } OutFileName :: Real (ref path) => { let mut file = fs :: File :: create_buffered (path) ? ; write_deps_to_file (& mut file) ? ; } } } ; match result { Ok (_) => { if sess . opts . json_artifact_notifications { sess . dcx () . emit_artifact_notification (deps_filename , "dep-info") ; } } Err (error) => { sess . dcx () . emit_fatal (errors :: ErrorWritingDependencies { path : deps_filename , error }) ; } } }
}

macro_rules! resolver_for_lowering_raw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function resolver_for_lowering_raw in module {}", module_path!());
    };
}

mkfn!{
    resolver_for_lowering_raw_introspect!();
    fn resolver_for_lowering_raw < 'tcx > (tcx : TyCtxt < 'tcx > , () : () ,) -> (& 'tcx Steal < (ty :: ResolverAstLowering , Arc < ast :: Crate >) > , & 'tcx ty :: ResolverGlobalCtxt) { let arenas = Resolver :: arenas () ; let _ = tcx . registered_tools (()) ; let (krate , pre_configured_attrs) = tcx . crate_for_resolver (()) . steal () ; let mut resolver = Resolver :: new (tcx , & pre_configured_attrs , krate . spans . inner_span , krate . spans . inject_use_span , & arenas ,) ; let krate = configure_and_expand (krate , & pre_configured_attrs , & mut resolver) ; tcx . untracked () . cstore . freeze () ; let ResolverOutputs { global_ctxt : untracked_resolutions , ast_lowering : untracked_resolver_for_lowering , } = resolver . into_outputs () ; let resolutions = tcx . arena . alloc (untracked_resolutions) ; (tcx . arena . alloc (Steal :: new ((untracked_resolver_for_lowering , Arc :: new (krate)))) , resolutions) }
}

macro_rules! write_dep_info_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_dep_info in module {}", module_path!());
    };
}

mkfn!{
    write_dep_info_introspect!();
    pub fn write_dep_info (tcx : TyCtxt < '_ >) { let _ = tcx . resolver_for_lowering () ; let sess = tcx . sess ; let _timer = sess . timer ("write_dep_info") ; let crate_name = tcx . crate_name (LOCAL_CRATE) ; let outputs = tcx . output_filenames (()) ; let output_paths = generated_output_paths (tcx , & outputs , sess . io . output_file . is_some () , crate_name) ; if let Some (input_path) = sess . io . input . opt_path () { if sess . opts . will_create_output_file () { if output_contains_path (& output_paths , input_path) { sess . dcx () . emit_fatal (errors :: InputFileWouldBeOverWritten { path : input_path }) ; } if let Some (dir_path) = output_conflicts_with_dir (& output_paths) { sess . dcx () . emit_fatal (errors :: GeneratedFileConflictsWithDirectory { input_path , dir_path , }) ; } } } if let Some (ref dir) = sess . io . temps_dir { if fs :: create_dir_all (dir) . is_err () { sess . dcx () . emit_fatal (errors :: TempsDirError) ; } } write_out_deps (tcx , & outputs , & output_paths) ; let only_dep_info = sess . opts . output_types . contains_key (& OutputType :: DepInfo) && sess . opts . output_types . len () == 1 ; if ! only_dep_info { if let Some (ref dir) = sess . io . output_dir { if fs :: create_dir_all (dir) . is_err () { sess . dcx () . emit_fatal (errors :: OutDirError) ; } } } }
}

macro_rules! write_interface_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_interface in module {}", module_path!());
    };
}

mkfn!{
    write_interface_introspect!();
    pub fn write_interface < 'tcx > (tcx : TyCtxt < 'tcx >) { if ! tcx . crate_types () . contains (& rustc_session :: config :: CrateType :: Sdylib) { return ; } let _timer = tcx . sess . timer ("write_interface") ; let (_ , krate) = & * tcx . resolver_for_lowering () . borrow () ; let krate = rustc_ast_pretty :: pprust :: print_crate_as_interface (krate , tcx . sess . psess . edition , & tcx . sess . psess . attr_id_generator ,) ; let export_output = tcx . output_filenames (()) . interface_path () ; let mut file = fs :: File :: create_buffered (export_output) . unwrap () ; if let Err (err) = write ! (file , "{}" , krate) { tcx . dcx () . fatal (format ! ("error writing interface file: {}" , err)) ; } }
}
mkitem!{pub static DEFAULT_QUERY_PROVIDERS : LazyLock < Providers > = LazyLock :: new (| | { let providers = & mut Providers :: default () ; providers . analysis = analysis ; providers . hir_crate = rustc_ast_lowering :: lower_to_hir ; providers . resolver_for_lowering_raw = resolver_for_lowering_raw ; providers . stripped_cfg_items = | tcx , _ | & tcx . resolutions (()) . stripped_cfg_items [..] ; providers . resolutions = | tcx , () | tcx . resolver_for_lowering_raw (()) . 1 ; providers . early_lint_checks = early_lint_checks ; providers . env_var_os = env_var_os ; limits :: provide (providers) ; proc_macro_decls :: provide (providers) ; rustc_const_eval :: provide (providers) ; rustc_middle :: hir :: provide (providers) ; rustc_borrowck :: provide (providers) ; rustc_incremental :: provide (providers) ; rustc_mir_build :: provide (providers) ; rustc_mir_transform :: provide (providers) ; rustc_monomorphize :: provide (providers) ; rustc_privacy :: provide (providers) ; rustc_query_impl :: provide (providers) ; rustc_resolve :: provide (providers) ; rustc_hir_analysis :: provide (providers) ; rustc_hir_typeck :: provide (providers) ; ty :: provide (providers) ; traits :: provide (providers) ; solve :: provide (providers) ; rustc_passes :: provide (providers) ; rustc_traits :: provide (providers) ; rustc_ty_utils :: provide (providers) ; rustc_metadata :: provide (providers) ; rustc_lint :: provide (providers) ; rustc_symbol_mangling :: provide (providers) ; rustc_codegen_ssa :: provide (providers) ; * providers }) ;}

macro_rules! create_and_enter_global_ctxt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_and_enter_global_ctxt in module {}", module_path!());
    };
}

mkfn!{
    create_and_enter_global_ctxt_introspect!();
    pub fn create_and_enter_global_ctxt < T , F : for < 'tcx > FnOnce (TyCtxt < 'tcx >) -> T > (compiler : & Compiler , krate : rustc_ast :: Crate , f : F ,) -> T { let sess = & compiler . sess ; let pre_configured_attrs = rustc_expand :: config :: pre_configure_attrs (sess , & krate . attrs) ; let crate_name = get_crate_name (sess , & pre_configured_attrs) ; let crate_types = collect_crate_types (sess , & pre_configured_attrs) ; let stable_crate_id = StableCrateId :: new (crate_name , crate_types . contains (& CrateType :: Executable) , sess . opts . cg . metadata . clone () , sess . cfg_version ,) ; let outputs = util :: build_output_filenames (& pre_configured_attrs , sess) ; let dep_type = DepsType { dep_names : rustc_query_impl :: dep_kind_names () } ; let dep_graph = setup_dep_graph (sess , crate_name , & dep_type) ; let cstore = FreezeLock :: new (Box :: new (CStore :: new (compiler . codegen_backend . metadata_loader ())) as _) ; let definitions = FreezeLock :: new (Definitions :: new (stable_crate_id)) ; let stable_crate_ids = FreezeLock :: new (StableCrateIdMap :: default ()) ; let untracked = Untracked { cstore , source_span : AppendOnlyIndexVec :: new () , definitions , stable_crate_ids } ; dep_graph . assert_ignored () ; let query_result_on_disk_cache = rustc_incremental :: load_query_result_cache (sess) ; let codegen_backend = & compiler . codegen_backend ; let mut providers = * DEFAULT_QUERY_PROVIDERS ; codegen_backend . provide (& mut providers) ; if let Some (callback) = compiler . override_queries { callback (sess , & mut providers) ; } let incremental = dep_graph . is_fully_enabled () ; let gcx_cell = OnceLock :: new () ; let arena = WorkerLocal :: new (| _ | Arena :: default ()) ; let hir_arena = WorkerLocal :: new (| _ | rustc_hir :: Arena :: default ()) ; let inner : Box < dyn for < 'tcx > FnOnce (& 'tcx Session , CurrentGcx , Arc < Proxy > , & 'tcx OnceLock < GlobalCtxt < 'tcx > > , & 'tcx WorkerLocal < Arena < 'tcx > > , & 'tcx WorkerLocal < rustc_hir :: Arena < 'tcx > > , F ,) -> T , > = Box :: new (move | sess , current_gcx , jobserver_proxy , gcx_cell , arena , hir_arena , f | { TyCtxt :: create_global_ctxt (gcx_cell , sess , crate_types , stable_crate_id , arena , hir_arena , untracked , dep_graph , rustc_query_impl :: query_callbacks (arena) , rustc_query_impl :: query_system (providers . queries , providers . extern_queries , query_result_on_disk_cache , incremental ,) , providers . hooks , current_gcx , jobserver_proxy , | tcx | { let feed = tcx . create_crate_num (stable_crate_id) . unwrap () ; assert_eq ! (feed . key () , LOCAL_CRATE) ; feed . crate_name (crate_name) ; let feed = tcx . feed_unit_query () ; feed . features_query (tcx . arena . alloc (rustc_expand :: config :: features (tcx . sess , & pre_configured_attrs , crate_name ,))) ; feed . crate_for_resolver (tcx . arena . alloc (Steal :: new ((krate , pre_configured_attrs)))) ; feed . output_filenames (Arc :: new (outputs)) ; let res = f (tcx) ; tcx . finish () ; res } ,) }) ; inner (& compiler . sess , compiler . current_gcx . clone () , Arc :: clone (& compiler . jobserver_proxy) , & gcx_cell , & arena , & hir_arena , f ,) }
}

macro_rules! run_required_analyses_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_required_analyses in module {}", module_path!());
    };
}

mkfn!{
    run_required_analyses_introspect!();
    # [doc = " Runs all analyses that we guarantee to run, even if errors were reported in earlier analyses."] # [doc = " This function never fails."] fn run_required_analyses (tcx : TyCtxt < '_ >) { if tcx . sess . opts . unstable_opts . input_stats { rustc_passes :: input_stats :: print_hir_stats (tcx) ; } # [cfg (all (not (doc) , debug_assertions))] rustc_passes :: hir_id_validator :: check_crate (tcx) ; tcx . ensure_done () . hir_crate_items (()) ; let sess = tcx . sess ; sess . time ("misc_checking_1" , | | { parallel ! ({ sess . time ("looking_for_entry_point" , || tcx . ensure_ok () . entry_fn (())) ; sess . time ("looking_for_derive_registrar" , || { tcx . ensure_ok () . proc_macro_decls_static (()) }) ; CStore :: from_tcx (tcx) . report_unused_deps (tcx) ; } , { tcx . ensure_ok () . exportable_items (LOCAL_CRATE) ; tcx . ensure_ok () . stable_order_of_exportable_impls (LOCAL_CRATE) ; tcx . par_hir_for_each_module (| module | { tcx . ensure_ok () . check_mod_attrs (module) ; tcx . ensure_ok () . check_mod_unstable_api_usage (module) ; }) ; } , { tcx . ensure_ok () . limits (()) ; }) ; }) ; rustc_hir_analysis :: check_crate (tcx) ; tcx . untracked () . definitions . freeze () ; sess . time ("MIR_borrow_checking" , | | { tcx . par_hir_body_owners (| def_id | { if ! tcx . is_typeck_child (def_id . to_def_id ()) { tcx . ensure_ok () . check_unsafety (def_id) ; tcx . ensure_ok () . mir_borrowck (def_id) ; tcx . ensure_ok () . check_transmutes (def_id) ; } tcx . ensure_ok () . has_ffi_unwind_calls (def_id) ; if tcx . sess . opts . output_types . should_codegen () || tcx . hir_body_const_context (def_id) . is_some () { tcx . ensure_ok () . mir_drops_elaborated_and_const_checked (def_id) ; } if tcx . is_coroutine (def_id . to_def_id ()) { tcx . ensure_ok () . mir_coroutine_witnesses (def_id) ; let _ = tcx . ensure_ok () . check_coroutine_obligations (tcx . typeck_root_def_id (def_id . to_def_id ()) . expect_local () ,) ; if ! tcx . is_async_drop_in_place_coroutine (def_id . to_def_id ()) { tcx . ensure_ok () . layout_of (ty :: TypingEnv :: post_analysis (tcx , def_id . to_def_id ()) . as_query_input (tcx . type_of (def_id) . instantiate_identity ()) ,) ; } } }) ; }) ; sess . time ("layout_testing" , | | layout_test :: test_layout (tcx)) ; sess . time ("abi_testing" , | | abi_test :: test_abi (tcx)) ; if tcx . sess . opts . unstable_opts . validate_mir { sess . time ("ensuring_final_MIR_is_computable" , | | { tcx . par_hir_body_owners (| def_id | { tcx . instance_mir (ty :: InstanceKind :: Item (def_id . into ())) ; }) ; }) ; } }
}

macro_rules! analysis_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function analysis in module {}", module_path!());
    };
}

mkfn!{
    analysis_introspect!();
    # [doc = " Runs the type-checking, region checking and other miscellaneous analysis"] # [doc = " passes on the crate."] fn analysis (tcx : TyCtxt < '_ > , () : ()) { run_required_analyses (tcx) ; let sess = tcx . sess ; if let Some (guar) = sess . dcx () . has_errors_excluding_lint_errors () { guar . raise_fatal () ; } sess . time ("misc_checking_3" , | | { parallel ! ({ tcx . ensure_ok () . effective_visibilities (()) ; parallel ! ({ tcx . par_hir_for_each_module (| module | { tcx . ensure_ok () . check_private_in_public (module) }) } , { tcx . par_hir_for_each_module (| module | { tcx . ensure_ok () . check_mod_deathness (module) }) ; } , { sess . time ("lint_checking" , || { rustc_lint :: check_crate (tcx) ; }) ; } , { tcx . ensure_ok () . clashing_extern_declarations (()) ; }) ; } , { sess . time ("privacy_checking_modules" , || { tcx . par_hir_for_each_module (| module | { tcx . ensure_ok () . check_mod_privacy (module) ; }) ; }) ; }) ; sess . time ("check_lint_expectations" , | | tcx . ensure_ok () . check_expectations (None)) ; let _ = tcx . all_diagnostic_items (()) ; }) ; }
}

macro_rules! start_codegen_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function start_codegen in module {}", module_path!());
    };
}

mkfn!{
    start_codegen_introspect!();
    # [doc = " Runs the codegen backend, after which the AST and analysis can"] # [doc = " be discarded."] pub (crate) fn start_codegen < 'tcx > (codegen_backend : & dyn CodegenBackend , tcx : TyCtxt < 'tcx > ,) -> (Box < dyn Any > , EncodedMetadata) { tcx . sess . timings . start_section (tcx . sess . dcx () , TimingSection :: Codegen) ; if let Some ((def_id , _)) = tcx . entry_fn (()) && tcx . has_attr (def_id , sym :: rustc_delayed_bug_from_inside_query) { tcx . ensure_ok () . trigger_delayed_bug (def_id) ; } if tcx . sess . opts . output_types . should_codegen () { rustc_symbol_mangling :: test :: report_symbol_names (tcx) ; } if let Some (guar) = tcx . sess . dcx () . has_errors_or_delayed_bugs () { guar . raise_fatal () ; } info ! ("Pre-codegen\n{:?}" , tcx . debug_stats ()) ; let metadata = rustc_metadata :: fs :: encode_and_write_metadata (tcx) ; let codegen = tcx . sess . time ("codegen_crate" , move | | codegen_backend . codegen_crate (tcx)) ; info ! ("Post-codegen\n{:?}" , tcx . debug_stats ()) ; if tcx . sess . opts . unstable_opts . print_type_sizes { tcx . sess . code_stats . print_type_sizes () ; } (codegen , metadata) }
}

macro_rules! get_crate_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_crate_name in module {}", module_path!());
    };
}

mkfn!{
    get_crate_name_introspect!();
    # [doc = " Compute and validate the crate name."] pub fn get_crate_name (sess : & Session , krate_attrs : & [ast :: Attribute]) -> Symbol { let attr_crate_name = parse_crate_name (sess , krate_attrs , ShouldEmit :: EarlyFatal { also_emit_lints : true }) ; let validate = | name , span | { rustc_session :: output :: validate_crate_name (sess , name , span) ; name } ; if let Some (crate_name) = & sess . opts . crate_name { let crate_name = Symbol :: intern (crate_name) ; if let Some ((attr_crate_name , span)) = attr_crate_name && attr_crate_name != crate_name { sess . dcx () . emit_err (errors :: CrateNameDoesNotMatch { span , crate_name , attr_crate_name , }) ; } return validate (crate_name , None) ; } if let Some ((crate_name , span)) = attr_crate_name { return validate (crate_name , Some (span)) ; } if let Input :: File (ref path) = sess . io . input && let Some (file_stem) = path . file_stem () . and_then (| s | s . to_str ()) { if file_stem . starts_with ('-') { sess . dcx () . emit_err (errors :: CrateNameInvalid { crate_name : file_stem }) ; } else { return validate (Symbol :: intern (& file_stem . replace ('-' , "_")) , None) ; } } sym :: rust_out }
}

macro_rules! parse_crate_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_crate_name in module {}", module_path!());
    };
}

mkfn!{
    parse_crate_name_introspect!();
    pub (crate) fn parse_crate_name (sess : & Session , attrs : & [ast :: Attribute] , emit_errors : ShouldEmit ,) -> Option < (Symbol , Span) > { let rustc_hir :: Attribute :: Parsed (AttributeKind :: CrateName { name , name_span , .. }) = AttributeParser :: parse_limited_should_emit (sess , & attrs , sym :: crate_name , DUMMY_SP , rustc_ast :: node_id :: CRATE_NODE_ID , None , emit_errors ,) ? else { unreachable ! ("crate_name is the only attr we could've parsed here") ; } ; Some ((name , name_span)) }
}

macro_rules! get_recursion_limit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_recursion_limit in module {}", module_path!());
    };
}

mkfn!{
    get_recursion_limit_introspect!();
    fn get_recursion_limit (krate_attrs : & [ast :: Attribute] , sess : & Session) -> Limit { let attr = AttributeParser :: parse_limited_should_emit (sess , & krate_attrs , sym :: recursion_limit , DUMMY_SP , rustc_ast :: node_id :: CRATE_NODE_ID , None , ShouldEmit :: EarlyFatal { also_emit_lints : false } ,) ; crate :: limits :: get_recursion_limit (attr . as_slice ()) }
}