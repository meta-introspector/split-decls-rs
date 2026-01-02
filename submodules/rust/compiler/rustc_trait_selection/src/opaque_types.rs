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
mkitem!{macro_rules ! emit_message { ($ ($ arg : tt) *) => { { use std :: fs :: OpenOptions ; use std :: io :: Write ; let message = format ! ($ ($ arg) *) ; if let Ok (mut file) = OpenOptions :: new () . create (true) . append (true) . open ("macro_report.txt") { let _ = writeln ! (file , "{}" , message) ; } } } ; }}
mkitem!{macro_rules ! mkfn { ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: catch_fatal_errors_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: catch_fatal_errors_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn parse_crate_attrs < $ lifetime : lifetime > ($ ($ param : tt) *) -> PResult < $ lifetime2 : lifetime , ast :: AttrVec > $ body : block) => { $ (#[$ attr]) * fn parse_crate_attrs < $ lifetime > ($ ($ param) *) -> PResult < $ lifetime2 , ast :: AttrVec > { $ introspect ; emit_message ! ("🚀 MARKER: parse_crate_attrs_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: parse_crate_attrs_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn init_logger_with_additional_layer < F , T > ($ ($ param : tt) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , $ body : block) => { $ (#[$ attr]) * fn init_logger_with_additional_layer < F , T > ($ ($ param) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , { $ introspect ; emit_message ! ("🚀 MARKER: init_logger_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: init_logger_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < F , T > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where F : FnOnce ($ ($ fnonce_args : tt) *) $ ($ where_rest : tt) * $ body : block) => { $ (#[$ attr]) * fn $ name < F , T > ($ ($ param) *) $ (-> $ ret) ? where F : FnOnce ($ ($ fnonce_args) *) $ ($ where_rest) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where $ ($ where_clause : tt) * $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? where $ ($ where_clause) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub ($ vis : ident) fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub ($ vis) fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_vis - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_vis - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; fn $ name : ident () $ body : block) => { fn $ name () { $ introspect ; emit_message ! ("🚀 MARKER: simple - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: simple - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ ($ anything : tt) *) => { $ ($ anything) * } ; }}
mkitem!{macro_rules ! safe_println { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{macro_rules ! safe_print { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{#[macro_export] macro_rules ! include_rust_compiler { ($ crate_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ crate_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_" , $ file , ".rs")) ; } ; ($ crate_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_lib.rs")) ; } ; }}
mkitem!{#[macro_export] macro_rules ! include_rust_library { ($ lib_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ lib_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_" , $ file , ".rs")) ; } ; ($ lib_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_lib.rs")) ; } ; }}
mkitem!{#[macro_export] macro_rules ! include_processed { ($ path : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_" , $ path , ".rs")) ; } ; }}
mkitem!{macro_rules ! mkinclude { ($ path : ident) => { } ; ($ path : literal) => { include ! ($ path) } ; }}
mkitem!{macro_rules ! mkitem { (include ! ($ path : ident) ;) => { } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ string_lit : literal }) => { $ macro_name :: $ macro_sub ! { $ string_lit } } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ ($ args : tt) * }) => { $ macro_name :: $ macro_sub ! { $ ($ args) * } } ; ($ macro_name : ident ! { $ ($ args : tt) * }) => { $ macro_name ! { $ ($ args) * } } ; ($ item : item) => { $ item } ; }}
mkitem!{#[macro_export] macro_rules ! mkmod { ($ name : ident , { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (pub mod $ name : ident { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; pub mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (mod $ name : ident { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; }}
mkitem!{#[macro_export] macro_rules ! mkuse { ($ use_stmt : item) => { emit_message ! ("USE|{}|{}" , module_path ! () , stringify ! ($ use_stmt)) ; $ use_stmt } ; }}
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
mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_hir :: OpaqueTyOrigin ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_infer :: infer :: outlives :: env :: OutlivesEnvironment ;}
mkuse!{use rustc_infer :: infer :: { InferCtxt , TyCtxtInferExt } ;}
mkuse!{use rustc_middle :: ty :: { self , DefiningScopeKind , GenericArgKind , GenericArgs , OpaqueTypeKey , Ty , TyCtxt , TypeVisitableExt , TypingMode , fold_regions , } ;}
mkuse!{use rustc_span :: { ErrorGuaranteed , Span } ;}
mkuse!{use crate :: errors :: NonGenericOpaqueTypeParam ;}
mkuse!{use crate :: regions :: OutlivesEnvironmentBuildExt ;}
mkuse!{use crate :: traits :: ObligationCtxt ;}
mkitem!{mkenum!{#[derive (Debug)] pub enum NonDefiningUseReason < 'tcx > { Tainted (ErrorGuaranteed) , NotAParam { opaque_type_key : OpaqueTypeKey < 'tcx > , param_index : usize , span : Span } , DuplicateParam { opaque_type_key : OpaqueTypeKey < 'tcx > , param_indices : Vec < usize > , span : Span } , }}}
mkitem!{mkimpl!{impl From < ErrorGuaranteed > for NonDefiningUseReason < '_ > { fn from (guar : ErrorGuaranteed) -> Self { NonDefiningUseReason :: Tainted (guar) } }}}
mkitem!{mkimpl!{impl < 'tcx > NonDefiningUseReason < 'tcx > { pub fn report (self , infcx : & InferCtxt < 'tcx >) -> ErrorGuaranteed { let tcx = infcx . tcx ; match self { NonDefiningUseReason :: Tainted (guar) => guar , NonDefiningUseReason :: NotAParam { opaque_type_key , param_index , span } => { let opaque_generics = tcx . generics_of (opaque_type_key . def_id) ; let opaque_param = opaque_generics . param_at (param_index , tcx) ; let kind = opaque_param . kind . descr () ; infcx . dcx () . emit_err (NonGenericOpaqueTypeParam { arg : opaque_type_key . args [param_index] , kind , span , param_span : tcx . def_span (opaque_param . def_id) , }) } NonDefiningUseReason :: DuplicateParam { opaque_type_key , param_indices , span } => { let opaque_generics = tcx . generics_of (opaque_type_key . def_id) ; let descr = opaque_generics . param_at (param_indices [0] , tcx) . kind . descr () ; let spans : Vec < _ > = param_indices . into_iter () . map (| i | tcx . def_span (opaque_generics . param_at (i , tcx) . def_id)) . collect () ; infcx . dcx () . struct_span_err (span , "non-defining opaque type use in defining scope") . with_span_note (spans , format ! ("{descr} used multiple times")) . emit () } } } }}}

macro_rules! opaque_type_has_defining_use_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function opaque_type_has_defining_use_args in module {}", module_path!());
    };
}

mkfn!{
    opaque_type_has_defining_use_args_introspect!();
    #[doc = " Opaque type parameter validity check as documented in the [rustc-dev-guide chapter]."] #[doc = " With the new solver, uses which fail this check are simply treated as non-defining"] #[doc = " and we only emit an error if no defining use exists."] #[doc = ""] #[doc = " [rustc-dev-guide chapter]:"] #[doc = " https://rustc-dev-guide.rust-lang.org/opaque-types-region-infer-restrictions.html"] pub fn opaque_type_has_defining_use_args < 'tcx > (infcx : & InferCtxt < 'tcx > , opaque_type_key : OpaqueTypeKey < 'tcx > , span : Span , defining_scope_kind : DefiningScopeKind ,) -> Result < () , NonDefiningUseReason < 'tcx > > { let tcx = infcx . tcx ; let opaque_env = LazyOpaqueTyEnv :: new (tcx , opaque_type_key . def_id) ; let mut seen_params : FxIndexMap < _ , Vec < _ > > = FxIndexMap :: default () ; if let DefiningScopeKind :: MirBorrowck = defining_scope_kind { infcx . tcx . type_of_opaque_hir_typeck (opaque_type_key . def_id) . instantiate_identity () . error_reported () ? ; } for (i , arg) in opaque_type_key . iter_captured_args (tcx) { let arg_is_param = match arg . kind () { GenericArgKind :: Lifetime (lt) => match defining_scope_kind { DefiningScopeKind :: HirTypeck => continue , DefiningScopeKind :: MirBorrowck => { matches ! (lt . kind () , ty :: ReEarlyParam (_) | ty :: ReLateParam (_)) || (lt . is_static () && opaque_env . param_equal_static (i)) } } , GenericArgKind :: Type (ty) => matches ! (ty . kind () , ty :: Param (_)) , GenericArgKind :: Const (ct) => matches ! (ct . kind () , ty :: ConstKind :: Param (_)) , } ; if arg_is_param { let seen_where = seen_params . entry (arg) . or_default () ; if ! seen_where . first () . is_some_and (| & prev_i | opaque_env . params_equal (i , prev_i)) { seen_where . push (i) ; } } else { opaque_env . param_is_error (i) ? ; return Err (NonDefiningUseReason :: NotAParam { opaque_type_key , param_index : i , span }) ; } } for (_ , param_indices) in seen_params { if param_indices . len () > 1 { return Err (NonDefiningUseReason :: DuplicateParam { opaque_type_key , param_indices , span , }) ; } } Ok (()) }
}
mkitem!{mkstruct!{#[doc = " Computes if an opaque type requires a lifetime parameter to be equal to"] #[doc = " another one or to the `'static` lifetime."] #[doc = " These requirements are derived from the explicit and implied bounds."] struct LazyOpaqueTyEnv < 'tcx > { tcx : TyCtxt < 'tcx > , def_id : LocalDefId , #[doc = " Equal parameters will have the same name. Computed Lazily."] #[doc = " Example:"] #[doc = "     `type Opaque<'a: 'static, 'b: 'c, 'c: 'b> = impl Sized;`"] #[doc = "     Identity args: `['a, 'b, 'c]`"] #[doc = "     Canonical args: `['static, 'b, 'b]`"] canonical_args : std :: cell :: OnceCell < ty :: GenericArgsRef < 'tcx > > , }}}
mkitem!{mkimpl!{impl < 'tcx > LazyOpaqueTyEnv < 'tcx > { fn new (tcx : TyCtxt < 'tcx > , def_id : LocalDefId) -> Self { Self { tcx , def_id , canonical_args : std :: cell :: OnceCell :: new () } } fn param_equal_static (& self , param_index : usize) -> bool { self . get_canonical_args () [param_index] . expect_region () . is_static () } fn params_equal (& self , param1 : usize , param2 : usize) -> bool { let canonical_args = self . get_canonical_args () ; canonical_args [param1] == canonical_args [param2] } fn param_is_error (& self , param_index : usize) -> Result < () , ErrorGuaranteed > { self . get_canonical_args () [param_index] . error_reported () } fn get_canonical_args (& self) -> ty :: GenericArgsRef < 'tcx > { if let Some (& canonical_args) = self . canonical_args . get () { return canonical_args ; } let & Self { tcx , def_id , .. } = self ; let origin = tcx . local_opaque_ty_origin (def_id) ; let parent = match origin { OpaqueTyOrigin :: FnReturn { parent , .. } | OpaqueTyOrigin :: AsyncFn { parent , .. } | OpaqueTyOrigin :: TyAlias { parent , .. } => parent , } ; let param_env = tcx . param_env (parent) ; let args = GenericArgs :: identity_for_item (tcx , parent) . extend_to (tcx , def_id . to_def_id () , | param , _ | { tcx . map_opaque_lifetime_to_parent_lifetime (param . def_id . expect_local ()) . into () } ,) ; let infcx = tcx . infer_ctxt () . build (TypingMode :: non_body_analysis ()) ; let ocx = ObligationCtxt :: new (& infcx) ; let wf_tys = ocx . assumed_wf_types (param_env , parent) . unwrap_or_else (| _ | { tcx . dcx () . span_delayed_bug (tcx . def_span (def_id) , "error getting implied bounds") ; Default :: default () }) ; let outlives_env = OutlivesEnvironment :: new (& infcx , parent , param_env , wf_tys) ; let mut seen = vec ! [tcx . lifetimes . re_static] ; let canonical_args = fold_regions (tcx , args , | r1 , _ | { if r1 . is_error () { r1 } else if let Some (& r2) = seen . iter () . find (| & & r2 | { let free_regions = outlives_env . free_region_map () ; free_regions . sub_free_regions (tcx , r1 , r2) && free_regions . sub_free_regions (tcx , r2 , r1) }) { r2 } else { seen . push (r1) ; r1 } }) ; self . canonical_args . set (canonical_args) . unwrap () ; canonical_args } }}}

macro_rules! report_item_does_not_constrain_error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_item_does_not_constrain_error in module {}", module_path!());
    };
}

mkfn!{
    report_item_does_not_constrain_error_introspect!();
    pub fn report_item_does_not_constrain_error < 'tcx > (tcx : TyCtxt < 'tcx > , item_def_id : LocalDefId , def_id : LocalDefId , non_defining_use : Option < (OpaqueTypeKey < 'tcx > , Span) > ,) -> ErrorGuaranteed { let span = tcx . def_ident_span (item_def_id) . unwrap_or_else (| | tcx . def_span (item_def_id)) ; let opaque_type_span = tcx . def_span (def_id) ; let opaque_type_name = tcx . def_path_str (def_id) ; let mut err = tcx . dcx () . struct_span_err (span , format ! ("item does not constrain `{opaque_type_name}`")) ; err . note ("consider removing `#[define_opaque]` or adding an empty `#[define_opaque()]`") ; err . span_note (opaque_type_span , "this opaque type is supposed to be constrained") ; if let Some ((key , span)) = non_defining_use { let opaque_ty = Ty :: new_opaque (tcx , key . def_id . into () , key . args) ; err . span_note (span , format ! ("this use of `{opaque_ty}` does not have unique universal generic arguments") ,) ; } err . emit () }
}