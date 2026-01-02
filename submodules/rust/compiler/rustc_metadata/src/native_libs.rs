mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use rustc_abi :: ExternAbi ;}
mkuse!{use rustc_ast :: CRATE_NODE_ID ;}
mkuse!{use rustc_attr_parsing :: { ShouldEmit , eval_config_entry } ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_hir :: attrs :: { AttributeKind , NativeLibKind , PeImportNameType } ;}
mkuse!{use rustc_hir :: find_attr ;}
mkuse!{use rustc_middle :: query :: LocalCrate ;}
mkuse!{use rustc_middle :: ty :: { self , List , Ty , TyCtxt } ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: config :: CrateType ;}
mkuse!{use rustc_session :: cstore :: { DllCallingConvention , DllImport , ForeignModule , NativeLib } ;}
mkuse!{use rustc_session :: search_paths :: PathKind ;}
mkuse!{use rustc_span :: Symbol ;}
mkuse!{use rustc_span :: def_id :: { DefId , LOCAL_CRATE } ;}
mkuse!{use rustc_target :: spec :: { BinaryFormat , LinkSelfContainedComponents } ;}
mkuse!{use crate :: errors ;}
mkitem!{mkstruct!{# [doc = " The fallback directories are passed to linker, but not used when rustc does the search,"] # [doc = " because in the latter case the set of fallback directories cannot always be determined"] # [doc = " consistently at the moment."] pub struct NativeLibSearchFallback < 'a > { pub self_contained_components : LinkSelfContainedComponents , pub apple_sdk_root : Option < & 'a Path > , }}}

macro_rules! walk_native_lib_search_dirs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function walk_native_lib_search_dirs in module {}", module_path!());
    };
}

mkfn!{
    walk_native_lib_search_dirs_introspect!();
    pub fn walk_native_lib_search_dirs < R > (sess : & Session , fallback : Option < NativeLibSearchFallback < '_ > > , mut f : impl FnMut (& Path , bool) -> ControlFlow < R > ,) -> ControlFlow < R > { for search_path in sess . target_filesearch () . cli_search_paths (PathKind :: Native) { f (& search_path . dir , false) ? ; } for search_path in sess . target_filesearch () . cli_search_paths (PathKind :: Framework) { if search_path . kind != PathKind :: All { f (& search_path . dir , true) ? ; } } let Some (NativeLibSearchFallback { self_contained_components , apple_sdk_root }) = fallback else { return ControlFlow :: Continue (()) ; } ; if self_contained_components . intersects (LinkSelfContainedComponents :: LIBC | LinkSelfContainedComponents :: UNWIND | LinkSelfContainedComponents :: MINGW ,) { f (& sess . target_tlib_path . dir . join ("self-contained") , false) ? ; } if sess . target . vendor == "fortanix" || sess . target . os == "linux" || sess . target . os == "fuchsia" || sess . target . is_like_aix || sess . target . is_like_darwin && ! sess . opts . unstable_opts . sanitizer . is_empty () { f (& sess . target_tlib_path . dir , false) ? ; } if let Some (sdk_root) = apple_sdk_root && sess . target . env == "macabi" { f (& sdk_root . join ("System/iOSSupport/usr/lib") , false) ? ; f (& sdk_root . join ("System/iOSSupport/System/Library/Frameworks") , true) ? ; } ControlFlow :: Continue (()) }
}

macro_rules! try_find_native_static_library_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_find_native_static_library in module {}", module_path!());
    };
}

mkfn!{
    try_find_native_static_library_introspect!();
    pub fn try_find_native_static_library (sess : & Session , name : & str , verbatim : bool ,) -> Option < PathBuf > { let default = sess . staticlib_components (verbatim) ; let formats = if verbatim { vec ! [default] } else { let unix = ("lib" , ".a") ; if default == unix { vec ! [default] } else { vec ! [default , unix] } } ; walk_native_lib_search_dirs (sess , None , | dir , is_framework | { if ! is_framework { for (prefix , suffix) in & formats { let test = dir . join (format ! ("{prefix}{name}{suffix}")) ; if test . exists () { return ControlFlow :: Break (test) ; } } } ControlFlow :: Continue (()) }) . break_value () }
}

macro_rules! try_find_native_dynamic_library_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_find_native_dynamic_library in module {}", module_path!());
    };
}

mkfn!{
    try_find_native_dynamic_library_introspect!();
    pub fn try_find_native_dynamic_library (sess : & Session , name : & str , verbatim : bool ,) -> Option < PathBuf > { let default = sess . staticlib_components (verbatim) ; let formats = if verbatim { vec ! [default] } else { let meson = ("lib" , ".dll.a") ; let mingw = ("lib" , ".a") ; vec ! [default , meson , mingw] } ; walk_native_lib_search_dirs (sess , None , | dir , is_framework | { if ! is_framework { for (prefix , suffix) in & formats { let test = dir . join (format ! ("{prefix}{name}{suffix}")) ; if test . exists () { return ControlFlow :: Break (test) ; } } } ControlFlow :: Continue (()) }) . break_value () }
}

macro_rules! find_native_static_library_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_native_static_library in module {}", module_path!());
    };
}

mkfn!{
    find_native_static_library_introspect!();
    pub fn find_native_static_library (name : & str , verbatim : bool , sess : & Session) -> PathBuf { try_find_native_static_library (sess , name , verbatim) . unwrap_or_else (| | sess . dcx () . emit_fatal (errors :: MissingNativeLibrary :: new (name , verbatim))) }
}

macro_rules! find_bundled_library_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_bundled_library in module {}", module_path!());
    };
}

mkfn!{
    find_bundled_library_introspect!();
    fn find_bundled_library (name : Symbol , verbatim : Option < bool > , kind : NativeLibKind , has_cfg : bool , tcx : TyCtxt < '_ > ,) -> Option < Symbol > { let sess = tcx . sess ; if let NativeLibKind :: Static { bundle : Some (true) | None , whole_archive } = kind && tcx . crate_types () . iter () . any (| t | matches ! (t , & CrateType :: Rlib | CrateType :: Staticlib)) && (sess . opts . unstable_opts . packed_bundled_libs || has_cfg || whole_archive == Some (true)) { let verbatim = verbatim . unwrap_or (false) ; return find_native_static_library (name . as_str () , verbatim , sess) . file_name () . and_then (| s | s . to_str ()) . map (Symbol :: intern) ; } None }
}

macro_rules! collect_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect in module {}", module_path!());
    };
}

mkfn!{
    collect_introspect!();
    pub (crate) fn collect (tcx : TyCtxt < '_ > , LocalCrate : LocalCrate) -> Vec < NativeLib > { let mut collector = Collector { tcx , libs : Vec :: new () } ; if tcx . sess . opts . unstable_opts . link_directives { for module in tcx . foreign_modules (LOCAL_CRATE) . values () { collector . process_module (module) ; } } collector . process_command_line () ; collector . libs }
}

macro_rules! relevant_lib_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function relevant_lib in module {}", module_path!());
    };
}

mkfn!{
    relevant_lib_introspect!();
    pub (crate) fn relevant_lib (sess : & Session , lib : & NativeLib) -> bool { match lib . cfg { Some (ref cfg) => { eval_config_entry (sess , cfg , CRATE_NODE_ID , None , ShouldEmit :: ErrorsAndLints) . as_bool () } None => true , } }
}
mkitem!{mkstruct!{struct Collector < 'tcx > { tcx : TyCtxt < 'tcx > , libs : Vec < NativeLib > , }}}
mkitem!{mkimpl!{impl < 'tcx > Collector < 'tcx > { fn process_module (& mut self , module : & ForeignModule) { let ForeignModule { def_id , abi , ref foreign_items } = * module ; let def_id = def_id . expect_local () ; let sess = self . tcx . sess ; if matches ! (abi , ExternAbi :: Rust) { return ; } for attr in find_attr ! (self . tcx . get_all_attrs (def_id) , AttributeKind :: Link (links , _) => links) . iter () . map (| v | v . iter ()) . flatten () { let dll_imports = match attr . kind { NativeLibKind :: RawDylib => foreign_items . iter () . map (| & child_item | { self . build_dll_import (abi , attr . import_name_type . map (| (import_name_type , _) | import_name_type) , child_item ,) }) . collect () , _ => { for & child_item in foreign_items { if let Some (span) = find_attr ! (self . tcx . get_all_attrs (child_item) , AttributeKind :: LinkOrdinal { span , .. } => * span) { sess . dcx () . emit_err (errors :: LinkOrdinalRawDylib { span }) ; } } Vec :: new () } } ; let filename = find_bundled_library (attr . name , attr . verbatim , attr . kind , attr . cfg . is_some () , self . tcx ,) ; self . libs . push (NativeLib { name : attr . name , filename , kind : attr . kind , cfg : attr . cfg . clone () , foreign_module : Some (def_id . to_def_id ()) , verbatim : attr . verbatim , dll_imports , }) ; } } fn process_command_line (& mut self) { let mut renames = FxHashSet :: default () ; for lib in & self . tcx . sess . opts . libs { if let NativeLibKind :: Framework { .. } = lib . kind && ! self . tcx . sess . target . is_like_darwin { self . tcx . dcx () . emit_err (errors :: LibFrameworkApple) ; } if let Some (ref new_name) = lib . new_name { let any_duplicate = self . libs . iter () . any (| n | n . name . as_str () == lib . name) ; if new_name . is_empty () { self . tcx . dcx () . emit_err (errors :: EmptyRenamingTarget { lib_name : & lib . name }) ; } else if ! any_duplicate { self . tcx . dcx () . emit_err (errors :: RenamingNoLink { lib_name : & lib . name }) ; } else if ! renames . insert (& lib . name) { self . tcx . dcx () . emit_err (errors :: MultipleRenamings { lib_name : & lib . name }) ; } } } for passed_lib in & self . tcx . sess . opts . libs { let mut existing = self . libs . extract_if (.. , | lib | { if lib . name . as_str () == passed_lib . name { if lib . has_modifiers () || passed_lib . has_modifiers () { match lib . foreign_module { Some (def_id) => { self . tcx . dcx () . emit_err (errors :: NoLinkModOverride { span : Some (self . tcx . def_span (def_id)) , }) } None => self . tcx . dcx () . emit_err (errors :: NoLinkModOverride { span : None }) , } ; } if passed_lib . kind != NativeLibKind :: Unspecified { lib . kind = passed_lib . kind ; } if let Some (new_name) = & passed_lib . new_name { lib . name = Symbol :: intern (new_name) ; } lib . verbatim = passed_lib . verbatim ; return true ; } false }) . collect :: < Vec < _ > > () ; if existing . is_empty () { let new_name : Option < & str > = passed_lib . new_name . as_deref () ; let name = Symbol :: intern (new_name . unwrap_or (& passed_lib . name)) ; let filename = find_bundled_library (name , passed_lib . verbatim , passed_lib . kind , false , self . tcx ,) ; self . libs . push (NativeLib { name , filename , kind : passed_lib . kind , cfg : None , foreign_module : None , verbatim : passed_lib . verbatim , dll_imports : Vec :: new () , }) ; } else { self . libs . append (& mut existing) ; } } } fn i686_arg_list_size (& self , item : DefId) -> usize { let argument_types : & List < Ty < '_ > > = self . tcx . instantiate_bound_regions_with_erased (self . tcx . type_of (item) . instantiate_identity () . fn_sig (self . tcx) . inputs () . map_bound (| slice | self . tcx . mk_type_list (slice)) ,) ; argument_types . iter () . map (| ty | { let layout = self . tcx . layout_of (ty :: TypingEnv :: fully_monomorphized () . as_query_input (ty)) . expect ("layout") . layout ; (layout . size () . bytes_usize () + 3) & ! 3 }) . sum () } fn build_dll_import (& self , abi : ExternAbi , import_name_type : Option < PeImportNameType > , item : DefId ,) -> DllImport { let span = self . tcx . def_span (item) ; assert ! (self . tcx . sess . target . is_abi_supported (abi)) ; let calling_convention = if self . tcx . sess . target . arch == "x86" { match abi { ExternAbi :: C { .. } | ExternAbi :: Cdecl { .. } => DllCallingConvention :: C , ExternAbi :: Stdcall { .. } => { DllCallingConvention :: Stdcall (self . i686_arg_list_size (item)) } ExternAbi :: System { .. } => { let c_variadic = self . tcx . type_of (item) . instantiate_identity () . fn_sig (self . tcx) . c_variadic () ; if c_variadic { DllCallingConvention :: C } else { DllCallingConvention :: Stdcall (self . i686_arg_list_size (item)) } } ExternAbi :: Fastcall { .. } => { DllCallingConvention :: Fastcall (self . i686_arg_list_size (item)) } ExternAbi :: Vectorcall { .. } => { DllCallingConvention :: Vectorcall (self . i686_arg_list_size (item)) } _ => { self . tcx . dcx () . emit_fatal (errors :: RawDylibUnsupportedAbi { span }) ; } } } else { match abi { ExternAbi :: C { .. } | ExternAbi :: Win64 { .. } | ExternAbi :: System { .. } => { DllCallingConvention :: C } _ => { self . tcx . dcx () . emit_fatal (errors :: RawDylibUnsupportedAbi { span }) ; } } } ; let codegen_fn_attrs = self . tcx . codegen_fn_attrs (item) ; let import_name_type = codegen_fn_attrs . link_ordinal . map_or (import_name_type , | ord | Some (PeImportNameType :: Ordinal (ord))) ; let name = codegen_fn_attrs . symbol_name . unwrap_or_else (| | self . tcx . item_name (item)) ; if self . tcx . sess . target . binary_format == BinaryFormat :: Elf { let name = name . as_str () ; if name . contains ('\0') { self . tcx . dcx () . emit_err (errors :: RawDylibMalformed { span }) ; } else if let Some ((left , right)) = name . split_once ('@') && (left . is_empty () || right . is_empty () || right . contains ('@')) { self . tcx . dcx () . emit_err (errors :: RawDylibMalformed { span }) ; } } DllImport { name , import_name_type , calling_convention , span , is_fn : self . tcx . def_kind (item) . is_fn_like () , } } }}}