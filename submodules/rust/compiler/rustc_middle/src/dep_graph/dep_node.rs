mkuse!{use std :: sync :: Mutex ;}
mkuse!{use std :: collections :: HashMap ;}
mkuse!{use std :: sync :: LazyLock ;}
mkitem!{static USE_MATRIX : LazyLock < Mutex < HashMap < String , Vec < String > > > > = LazyLock :: new (| | Mutex :: new (HashMap :: new ())) ;}

macro_rules! get_use_matrix_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_use_matrix in module {}", module_path!());
    };
}

mkfn!{
    get_use_matrix_introspect!();
    pub fn get_use_matrix () -> HashMap < String , Vec < String > > { USE_MATRIX . lock () . unwrap () . clone () }
}
mkitem!{macro_rules ! emit_message { ($ ($ arg : tt) *) => { } ; }}
mkitem!{macro_rules ! mkfn { ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: catch_fatal_errors_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: catch_fatal_errors_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn parse_crate_attrs < $ lifetime : lifetime > ($ ($ param : tt) *) -> PResult < $ lifetime2 : lifetime , ast :: AttrVec > $ body : block) => { $ (# [$ attr]) * fn parse_crate_attrs < $ lifetime > ($ ($ param) *) -> PResult < $ lifetime2 , ast :: AttrVec > { $ introspect ; emit_message ! ("🚀 MARKER: parse_crate_attrs_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: parse_crate_attrs_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn init_logger_with_additional_layer < F , T > ($ ($ param : tt) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , $ body : block) => { $ (# [$ attr]) * fn init_logger_with_additional_layer < F , T > ($ ($ param) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , { $ introspect ; emit_message ! ("🚀 MARKER: init_logger_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: init_logger_specific") ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < F , T > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where F : FnOnce ($ ($ fnonce_args : tt) *) $ ($ where_rest : tt) * $ body : block) => { $ (# [$ attr]) * fn $ name < F , T > ($ ($ param) *) $ (-> $ ret) ? where F : FnOnce ($ ($ fnonce_args) *) $ ($ where_rest) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where $ ($ where_clause : tt) * $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? where $ ($ where_clause) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub ($ vis : ident) fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub ($ vis) fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_vis - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_vis - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * pub fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * pub fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (# [$ attr : meta]) * fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (# [$ attr]) * fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; fn $ name : ident () $ body : block) => { fn $ name () { $ introspect ; emit_message ! ("🚀 MARKER: simple - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: simple - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ ($ anything : tt) *) => { $ ($ anything) * } ; }}
mkitem!{macro_rules ! safe_println { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{macro_rules ! safe_print { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{# [macro_export] macro_rules ! include_rust_compiler { ($ crate_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ crate_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_" , $ file , ".rs")) ; } ; ($ crate_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_lib.rs")) ; } ; }}
mkitem!{# [macro_export] macro_rules ! include_rust_library { ($ lib_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ lib_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_" , $ file , ".rs")) ; } ; ($ lib_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_lib.rs")) ; } ; }}
mkitem!{# [macro_export] macro_rules ! include_processed { ($ path : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_" , $ path , ".rs")) ; } ; }}
mkitem!{macro_rules ! mkinclude { ($ path : ident) => { } ; ($ path : literal) => { include ! ($ path) } ; }}
mkitem!{macro_rules ! mkitem { (include ! ($ path : ident) ;) => { } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ string_lit : literal }) => { $ macro_name :: $ macro_sub ! { $ string_lit } } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ ($ args : tt) * }) => { $ macro_name :: $ macro_sub ! { $ ($ args) * } } ; ($ macro_name : ident ! { $ ($ args : tt) * }) => { $ macro_name ! { $ ($ args) * } } ; ($ item : item) => { $ item } ; }}
mkitem!{# [macro_export] macro_rules ! mkmod { ($ name : ident , { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (pub mod $ name : ident { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; pub mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (mod $ name : ident { $ ($ content : tt) * }) => { compile_error ! (concat ! ("MOD|" , module_path ! () , "|" , stringify ! ($ name))) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; }}
mkitem!{# [macro_export] macro_rules ! mkuse { ($ use_stmt : item) => { compile_error ! (concat ! ("USE|" , module_path ! () , "|" , stringify ! ($ use_stmt))) ; } ; }}
mkitem!{macro_rules ! mkstruct { ($ struct_def : item) => { $ struct_def } ; }}
mkitem!{macro_rules ! mkenum { ($ enum_def : item) => { $ enum_def } ; }}
mkitem!{macro_rules ! mktrait { ($ trait_def : item) => { $ trait_def } ; }}
mkitem!{macro_rules ! mkimpl { ($ impl_def : item) => { $ impl_def } ; }}
mkitem!{macro_rules ! getname { ($ name : ident) => { stringify ! ($ name) } ; }}
mkitem!{macro_rules ! getsrc { ($ name : ident) => { "processed file" } ; }}
mkitem!{macro_rules ! getpath { ($ name : ident) => { "processed_path" } ; }}
mkitem!{macro_rules ! get_deps { ($ name : ident) => { vec ! [] } ; }}
mkmod!{rustc_complete, { 
                getname!(rustc_complete);
                getsrc!(rustc_complete);
                getpath!(rustc_complete);
                get_deps!(rustc_complete);
                get_crates!(rustc_complete);
                mkinclude!(rustc_complete);
                mkmod!{emitter, { 
                getname!(emitter);
                getsrc!(emitter);
                getpath!(emitter);
                get_deps!(emitter);
                get_crates!(emitter);
                mkinclude!(emitter);
                
macro_rules! stderr_destination_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stderr_destination in module {}", module_path!());
    };
}

mkfn!{
    stderr_destination_introspect!();
    pub fn stderr_destination () { }
} 
            }}
mkmod!{registry, { 
                getname!(registry);
                getsrc!(registry);
                getpath!(registry);
                get_deps!(registry);
                get_crates!(registry);
                mkinclude!(registry);
                mkitem!{mkstruct!{pub struct Registry ;}} 
            }}
mkmod!{translation, { 
                getname!(translation);
                getsrc!(translation);
                getpath!(translation);
                get_deps!(translation);
                get_crates!(translation);
                mkinclude!(translation);
                mkitem!{mkstruct!{pub struct Translator ;}} 
            }}
mkitem!{mkstruct!{pub struct ColorConfig ;}}
mkitem!{mkstruct!{pub struct DiagCtxt ;}}
mkitem!{mkstruct!{pub struct ErrCode ;}}
mkitem!{mkstruct!{pub struct FatalError ;}}
mkitem!{mkstruct!{pub struct PResult < T > (pub T) ;}}
mkmod!{markdown, { 
                getname!(markdown);
                getsrc!(markdown);
                getpath!(markdown);
                get_deps!(markdown);
                get_crates!(markdown);
                mkinclude!(markdown);
                 
            }}
mkmod!{config, { 
                getname!(config);
                getsrc!(config);
                getpath!(config);
                get_deps!(config);
                get_crates!(config);
                mkinclude!(config);
                mkitem!{mkstruct!{pub struct CG_OPTIONS ;}}
mkitem!{mkstruct!{pub struct CrateType ;}}
mkitem!{mkstruct!{pub struct ErrorOutputType ;}}
mkitem!{mkstruct!{pub struct Input ;}}
mkitem!{mkstruct!{pub struct OptionDesc ;}}
mkitem!{mkstruct!{pub struct OutFileName ;}}
mkitem!{mkstruct!{pub struct OutputType ;}}
mkitem!{mkstruct!{pub struct Sysroot ;}}
mkitem!{mkstruct!{pub struct UnstableOptions ;}}
mkitem!{mkstruct!{pub struct Z_OPTIONS ;}}

macro_rules! nightly_options_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function nightly_options in module {}", module_path!());
    };
}

mkfn!{
    nightly_options_introspect!();
    pub fn nightly_options () { }
}

macro_rules! parse_target_triple_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_target_triple in module {}", module_path!());
    };
}

mkfn!{
    parse_target_triple_introspect!();
    pub fn parse_target_triple () { }
} 
            }}
mkmod!{getopts, { 
                getname!(getopts);
                getsrc!(getopts);
                getpath!(getopts);
                get_deps!(getopts);
                get_crates!(getopts);
                mkinclude!(getopts);
                mkitem!{mkstruct!{pub struct Matches ;}} 
            }}
mkmod!{lint, { 
                getname!(lint);
                getsrc!(lint);
                getpath!(lint);
                get_deps!(lint);
                get_crates!(lint);
                mkinclude!(lint);
                mkitem!{mkstruct!{pub struct Lint ;}}
mkitem!{mkstruct!{pub struct LintId ;}} 
            }}
mkmod!{output, { 
                getname!(output);
                getsrc!(output);
                getpath!(output);
                get_deps!(output);
                get_crates!(output);
                mkinclude!(output);
                mkitem!{mkstruct!{pub struct CRATE_TYPES ;}}

macro_rules! collect_crate_types_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_crate_types in module {}", module_path!());
    };
}

mkfn!{
    collect_crate_types_introspect!();
    pub fn collect_crate_types () { }
}

macro_rules! invalid_output_for_target_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function invalid_output_for_target in module {}", module_path!());
    };
}

mkfn!{
    invalid_output_for_target_introspect!();
    pub fn invalid_output_for_target () { }
} 
            }}
mkitem!{mkstruct!{pub struct EarlyDiagCtxt ;}}
mkitem!{mkstruct!{pub struct Session ;}}
mkitem!{mkstruct!{pub struct FileName ;}}
mkmod!{def_id, { 
                getname!(def_id);
                getsrc!(def_id);
                getpath!(def_id);
                get_deps!(def_id);
                get_crates!(def_id);
                mkinclude!(def_id);
                mkitem!{mkstruct!{pub struct LOCAL_CRATE ;}} 
            }}
mkmod!{ty, { 
                getname!(ty);
                getsrc!(ty);
                getpath!(ty);
                get_deps!(ty);
                get_crates!(ty);
                mkinclude!(ty);
                mkitem!{mkstruct!{pub struct TyCtxt < T > (pub T) ;}} 
            }} 
            }}
mkmod!{session_diagnostics, { 
                getname!(session_diagnostics);
                getsrc!(session_diagnostics);
                getpath!(session_diagnostics);
                get_deps!(session_diagnostics);
                get_crates!(session_diagnostics);
                mkinclude!(session_diagnostics);
                mkitem!{mkstruct!{pub struct CantEmitMIR ;}}
mkitem!{mkstruct!{pub struct RLinkEmptyVersionNumber ;}}
mkitem!{mkstruct!{pub struct RLinkEncodingVersionMismatch ;}}
mkitem!{mkstruct!{pub struct RLinkRustcVersionMismatch ;}}
mkitem!{mkstruct!{pub struct RLinkWrongFileType ;}}
mkitem!{mkstruct!{pub struct RlinkCorruptFile ;}}
mkitem!{mkstruct!{pub struct RlinkNotAFile ;}}
mkitem!{mkstruct!{pub struct RlinkUnableToRead ;}}
mkitem!{mkstruct!{pub struct UnstableFeatureUsage ;}} 
            }}
mkitem!{macro_rules ! do_not_use_print { ($ ($ t : tt) *) => { compile_error ! ("Don't use print") } ; }}
mkitem!{macro_rules ! do_not_use_safe_print { ($ ($ t : tt) *) => { compile_error ! ("Don't use safe_print") } ; }}
mkitem!{macro_rules ! mktrait { ($ trait_def : item) => { $ trait_def } ; }}
mkitem!{macro_rules ! mkimpl { ($ impl_def : item) => { $ impl_def } ; }}
mkitem!{macro_rules ! getname { ($ name : ident) => { pub fn get_module_name () -> &'static str { stringify ! ($ name) } } ; }}
mkitem!{macro_rules ! getsrc { ($ name : ident) => { pub fn get_source_info () -> &'static str { concat ! ("Module: " , stringify ! ($ name)) } } ; }}
mkitem!{macro_rules ! getpath { ($ name : ident) => { pub fn get_module_path () -> &'static str { module_path ! () } } ; }}
mkitem!{macro_rules ! get_deps { ($ name : ident) => { pub fn get_dependencies () -> &'static [&'static str] { & [] } } ; }}
mkitem!{macro_rules ! get_crates { ($ name : ident) => { pub fn get_required_crates () -> &'static [&'static str] { & [] } } ; }}
mkitem!{macro_rules ! forall_crates { ($ ($ crate_name : ident) ,*) => { $ (extern crate $ crate_name ;) * } ; }}
mkitem!{macro_rules ! emit_extern { ($ crate_name : ident) => { extern crate $ crate_name ; } ; }}
mkitem!{macro_rules ! get_externs { ($ crate_name : ident) => { stringify ! ($ crate_name) } ; }}
mkuse!{use rustc_data_structures :: fingerprint :: Fingerprint ;}
mkuse!{use rustc_hir :: def_id :: { CrateNum , DefId , LOCAL_CRATE , LocalDefId , LocalModDefId , ModDefId } ;}
mkuse!{use rustc_hir :: definitions :: DefPathHash ;}
mkuse!{use rustc_hir :: { HirId , ItemLocalId , OwnerId } ;}
mkuse!{pub use rustc_query_system :: dep_graph :: DepNode ;}
mkuse!{use rustc_query_system :: dep_graph :: FingerprintStyle ;}
mkuse!{pub use rustc_query_system :: dep_graph :: dep_node :: DepKind ;}
mkuse!{pub (crate) use rustc_query_system :: dep_graph :: { DepContext , DepNodeParams } ;}
mkuse!{use rustc_span :: Symbol ;}
mkuse!{use crate :: mir :: mono :: MonoItem ;}
mkuse!{use crate :: ty :: TyCtxt ;}
mkitem!{macro_rules ! define_dep_nodes { ($ ($ (# [$ attr : meta]) * [$ ($ modifiers : tt) *] fn $ variant : ident ($ ($ K : tt) *) -> $ V : ty ,) *) => { # [macro_export] macro_rules ! make_dep_kind_array { ($ mod : ident) => { [$ ($ mod ::$ variant ()) ,*] } ; } # [macro_export] macro_rules ! make_dep_kind_name_array { ($ mod : ident) => { vec ! { $ (*$ mod ::$ variant () . name) ,* } } ; } # [doc = " This enum serves as an index into arrays built by `make_dep_kind_array`."] # [allow (non_camel_case_types)] # [repr (u16)] enum DepKindDefs { $ ($ (# [$ attr]) * $ variant) ,* } # [allow (non_upper_case_globals)] pub mod dep_kinds { use super ::*; $ (pub const $ variant : DepKind = DepKind :: new (DepKindDefs ::$ variant as u16) ;) * } pub (crate) const DEP_KIND_VARIANTS : u16 = { let deps = & [$ (dep_kinds ::$ variant ,) *] ; let mut i = 0 ; while i < deps . len () { if i != deps [i] . as_usize () { panic ! () ; } i += 1 ; } deps . len () as u16 } ; pub (super) fn dep_kind_from_label_string (label : & str) -> Result < DepKind , () > { match label { $ (stringify ! ($ variant) => Ok (dep_kinds ::$ variant) ,) * _ => Err (()) , } } # [doc = " Contains variant => str representations for constructing"] # [doc = " DepNode groups for tests."] # [allow (dead_code , non_upper_case_globals)] pub mod label_strs { $ (pub const $ variant : & str = stringify ! ($ variant) ;) * } } ; }}
mkitem!{rustc_with_all_queries ! (define_dep_nodes ! [# [doc = " We use this for most things when incr. comp. is turned off."] [] fn Null () -> () , # [doc = " We use this to create a forever-red node."] [] fn Red () -> () , [] fn SideEffect () -> () , [] fn AnonZeroDeps () -> () , [] fn TraitSelect () -> () , [] fn CompileCodegenUnit () -> () , [] fn CompileMonoItem () -> () , [] fn Metadata () -> () ,]) ;}

macro_rules! make_compile_codegen_unit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_compile_codegen_unit in module {}", module_path!());
    };
}

mkfn!{
    make_compile_codegen_unit_introspect!();
    pub (crate) fn make_compile_codegen_unit (tcx : TyCtxt < '_ > , name : Symbol) -> DepNode { DepNode :: construct (tcx , dep_kinds :: CompileCodegenUnit , & name) }
}

macro_rules! make_compile_mono_item_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_compile_mono_item in module {}", module_path!());
    };
}

mkfn!{
    make_compile_mono_item_introspect!();
    pub (crate) fn make_compile_mono_item < 'tcx > (tcx : TyCtxt < 'tcx > , mono_item : & MonoItem < 'tcx > ,) -> DepNode { DepNode :: construct (tcx , dep_kinds :: CompileMonoItem , mono_item) }
}

macro_rules! make_metadata_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_metadata in module {}", module_path!());
    };
}

mkfn!{
    make_metadata_introspect!();
    pub (crate) fn make_metadata (tcx : TyCtxt < '_ >) -> DepNode { DepNode :: construct (tcx , dep_kinds :: Metadata , & ()) }
}
mkitem!{mktrait!{pub trait DepNodeExt : Sized { fn extract_def_id (& self , tcx : TyCtxt < '_ >) -> Option < DefId > ; fn from_label_string (tcx : TyCtxt < '_ > , label : & str , def_path_hash : DefPathHash ,) -> Result < Self , () > ; fn has_label_string (label : & str) -> bool ; }}}
mkitem!{mkimpl!{impl DepNodeExt for DepNode { # [doc = " Extracts the DefId corresponding to this DepNode. This will work"] # [doc = " if two conditions are met:"] # [doc = ""] # [doc = " 1. The Fingerprint of the DepNode actually is a DefPathHash, and"] # [doc = " 2. the item that the DefPath refers to exists in the current tcx."] # [doc = ""] # [doc = " Condition (1) is determined by the DepKind variant of the"] # [doc = " DepNode. Condition (2) might not be fulfilled if a DepNode"] # [doc = " refers to something from the previous compilation session that"] # [doc = " has been removed."] fn extract_def_id (& self , tcx : TyCtxt < '_ >) -> Option < DefId > { if tcx . fingerprint_style (self . kind) == FingerprintStyle :: DefPathHash { tcx . def_path_hash_to_def_id (DefPathHash (self . hash . into ())) } else { None } } # [doc = " Used in testing"] fn from_label_string (tcx : TyCtxt < '_ > , label : & str , def_path_hash : DefPathHash ,) -> Result < DepNode , () > { let kind = dep_kind_from_label_string (label) ? ; match tcx . fingerprint_style (kind) { FingerprintStyle :: Opaque | FingerprintStyle :: HirId => Err (()) , FingerprintStyle :: Unit => Ok (DepNode :: new_no_params (tcx , kind)) , FingerprintStyle :: DefPathHash => { Ok (DepNode :: from_def_path_hash (tcx , def_path_hash , kind)) } } } # [doc = " Used in testing"] fn has_label_string (label : & str) -> bool { dep_kind_from_label_string (label) . is_ok () } }}}
mkitem!{mkimpl!{impl < 'tcx > DepNodeParams < TyCtxt < 'tcx > > for () { # [inline (always)] fn fingerprint_style () -> FingerprintStyle { FingerprintStyle :: Unit } # [inline (always)] fn to_fingerprint (& self , _ : TyCtxt < 'tcx >) -> Fingerprint { Fingerprint :: ZERO } # [inline (always)] fn recover (_ : TyCtxt < 'tcx > , _ : & DepNode) -> Option < Self > { Some (()) } }}}
mkitem!{mkimpl!{impl < 'tcx > DepNodeParams < TyCtxt < 'tcx > > for DefId { # [inline (always)] fn fingerprint_style () -> FingerprintStyle { FingerprintStyle :: DefPathHash } # [inline (always)] fn to_fingerprint (& self , tcx : TyCtxt < 'tcx >) -> Fingerprint { tcx . def_path_hash (* self) . 0 } # [inline (always)] fn to_debug_str (& self , tcx : TyCtxt < 'tcx >) -> String { tcx . def_path_str (* self) } # [inline (always)] fn recover (tcx : TyCtxt < 'tcx > , dep_node : & DepNode) -> Option < Self > { dep_node . extract_def_id (tcx) } }}}
mkitem!{mkimpl!{impl < 'tcx > DepNodeParams < TyCtxt < 'tcx > > for LocalDefId { # [inline (always)] fn fingerprint_style () -> FingerprintStyle { FingerprintStyle :: DefPathHash } # [inline (always)] fn to_fingerprint (& self , tcx : TyCtxt < 'tcx >) -> Fingerprint { self . to_def_id () . to_fingerprint (tcx) } # [inline (always)] fn to_debug_str (& self , tcx : TyCtxt < 'tcx >) -> String { self . to_def_id () . to_debug_str (tcx) } # [inline (always)] fn recover (tcx : TyCtxt < 'tcx > , dep_node : & DepNode) -> Option < Self > { dep_node . extract_def_id (tcx) . map (| id | id . expect_local ()) } }}}
mkitem!{mkimpl!{impl < 'tcx > DepNodeParams < TyCtxt < 'tcx > > for OwnerId { # [inline (always)] fn fingerprint_style () -> FingerprintStyle { FingerprintStyle :: DefPathHash } # [inline (always)] fn to_fingerprint (& self , tcx : TyCtxt < 'tcx >) -> Fingerprint { self . to_def_id () . to_fingerprint (tcx) } # [inline (always)] fn to_debug_str (& self , tcx : TyCtxt < 'tcx >) -> String { self . to_def_id () . to_debug_str (tcx) } # [inline (always)] fn recover (tcx : TyCtxt < 'tcx > , dep_node : & DepNode) -> Option < Self > { dep_node . extract_def_id (tcx) . map (| id | OwnerId { def_id : id . expect_local () }) } }}}
mkitem!{mkimpl!{impl < 'tcx > DepNodeParams < TyCtxt < 'tcx > > for CrateNum { # [inline (always)] fn fingerprint_style () -> FingerprintStyle { FingerprintStyle :: DefPathHash } # [inline (always)] fn to_fingerprint (& self , tcx : TyCtxt < 'tcx >) -> Fingerprint { let def_id = self . as_def_id () ; def_id . to_fingerprint (tcx) } # [inline (always)] fn to_debug_str (& self , tcx : TyCtxt < 'tcx >) -> String { tcx . crate_name (* self) . to_string () } # [inline (always)] fn recover (tcx : TyCtxt < 'tcx > , dep_node : & DepNode) -> Option < Self > { dep_node . extract_def_id (tcx) . map (| id | id . krate) } }}}
mkitem!{mkimpl!{impl < 'tcx > DepNodeParams < TyCtxt < 'tcx > > for (DefId , DefId) { # [inline (always)] fn fingerprint_style () -> FingerprintStyle { FingerprintStyle :: Opaque } # [inline (always)] fn to_fingerprint (& self , tcx : TyCtxt < 'tcx >) -> Fingerprint { let (def_id_0 , def_id_1) = * self ; let def_path_hash_0 = tcx . def_path_hash (def_id_0) ; let def_path_hash_1 = tcx . def_path_hash (def_id_1) ; def_path_hash_0 . 0 . combine (def_path_hash_1 . 0) } # [inline (always)] fn to_debug_str (& self , tcx : TyCtxt < 'tcx >) -> String { let (def_id_0 , def_id_1) = * self ; format ! ("({}, {})" , tcx . def_path_debug_str (def_id_0) , tcx . def_path_debug_str (def_id_1)) } }}}
mkitem!{mkimpl!{impl < 'tcx > DepNodeParams < TyCtxt < 'tcx > > for HirId { # [inline (always)] fn fingerprint_style () -> FingerprintStyle { FingerprintStyle :: HirId } # [inline (always)] fn to_fingerprint (& self , tcx : TyCtxt < 'tcx >) -> Fingerprint { let HirId { owner , local_id } = * self ; let def_path_hash = tcx . def_path_hash (owner . to_def_id ()) ; Fingerprint :: new (def_path_hash . local_hash () , local_id . as_u32 () as u64 ,) } # [inline (always)] fn to_debug_str (& self , tcx : TyCtxt < 'tcx >) -> String { let HirId { owner , local_id } = * self ; format ! ("{}.{}" , tcx . def_path_str (owner) , local_id . as_u32 ()) } # [inline (always)] fn recover (tcx : TyCtxt < 'tcx > , dep_node : & DepNode) -> Option < Self > { if tcx . fingerprint_style (dep_node . kind) == FingerprintStyle :: HirId { let (local_hash , local_id) = Fingerprint :: from (dep_node . hash) . split () ; let def_path_hash = DefPathHash :: new (tcx . stable_crate_id (LOCAL_CRATE) , local_hash) ; let def_id = tcx . def_path_hash_to_def_id (def_path_hash) ? . expect_local () ; let local_id = local_id . as_u64 () . try_into () . unwrap_or_else (| _ | panic ! ("local id should be u32, found {local_id:?}")) ; Some (HirId { owner : OwnerId { def_id } , local_id : ItemLocalId :: from_u32 (local_id) }) } else { None } } }}}
mkitem!{mkimpl!{impl < 'tcx > DepNodeParams < TyCtxt < 'tcx > > for ModDefId { # [inline (always)] fn fingerprint_style () -> FingerprintStyle { FingerprintStyle :: DefPathHash } # [inline (always)] fn to_fingerprint (& self , tcx : TyCtxt < 'tcx >) -> Fingerprint { self . to_def_id () . to_fingerprint (tcx) } # [inline (always)] fn to_debug_str (& self , tcx : TyCtxt < 'tcx >) -> String { self . to_def_id () . to_debug_str (tcx) } # [inline (always)] fn recover (tcx : TyCtxt < 'tcx > , dep_node : & DepNode) -> Option < Self > { DefId :: recover (tcx , dep_node) . map (ModDefId :: new_unchecked) } }}}
mkitem!{mkimpl!{impl < 'tcx > DepNodeParams < TyCtxt < 'tcx > > for LocalModDefId { # [inline (always)] fn fingerprint_style () -> FingerprintStyle { FingerprintStyle :: DefPathHash } # [inline (always)] fn to_fingerprint (& self , tcx : TyCtxt < 'tcx >) -> Fingerprint { self . to_def_id () . to_fingerprint (tcx) } # [inline (always)] fn to_debug_str (& self , tcx : TyCtxt < 'tcx >) -> String { self . to_def_id () . to_debug_str (tcx) } # [inline (always)] fn recover (tcx : TyCtxt < 'tcx > , dep_node : & DepNode) -> Option < Self > { LocalDefId :: recover (tcx , dep_node) . map (LocalModDefId :: new_unchecked) } }}}