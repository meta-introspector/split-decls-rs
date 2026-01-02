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
mkuse!{use hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: traits :: ObligationCause ;}
mkuse!{use rustc_middle :: traits :: solve :: Goal ;}
mkuse!{use rustc_middle :: ty :: error :: { ExpectedFound , TypeError } ;}
mkuse!{use rustc_middle :: ty :: { self , BottomUpFolder , OpaqueHiddenType , OpaqueTypeKey , Ty , TyCtxt , TypeFoldable , TypeVisitableExt , } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use super :: { DefineOpaqueTypes , RegionVariableOrigin } ;}
mkuse!{use crate :: errors :: OpaqueHiddenTypeDiag ;}
mkuse!{use crate :: infer :: { InferCtxt , InferOk } ;}
mkuse!{use crate :: traits :: { self , Obligation , PredicateObligations } ;}
mkmod!{table, { 
                getname!(table);
                getsrc!(table);
                getpath!(table);
                get_deps!(table);
                get_crates!(table);
                mkinclude!(table);
                 
            }}
mkuse!{pub use table :: { OpaqueTypeStorage , OpaqueTypeStorageEntries , OpaqueTypeTable } ;}
mkitem!{mkimpl!{impl < 'tcx > InferCtxt < 'tcx > { #[doc = " This is a backwards compatibility hack to prevent breaking changes from"] #[doc = " lazy TAIT around RPIT handling."] pub fn replace_opaque_types_with_inference_vars < T : TypeFoldable < TyCtxt < 'tcx > > > (& self , value : T , body_id : LocalDefId , span : Span , param_env : ty :: ParamEnv < 'tcx > ,) -> InferOk < 'tcx , T > { if self . next_trait_solver () { return InferOk { value , obligations : PredicateObligations :: new () } ; } if ! value . has_opaque_types () { return InferOk { value , obligations : PredicateObligations :: new () } ; } let mut obligations = PredicateObligations :: new () ; let value = value . fold_with (& mut BottomUpFolder { tcx : self . tcx , lt_op : | lt | lt , ct_op : | ct | ct , ty_op : | ty | match * ty . kind () { ty :: Alias (ty :: Opaque , ty :: AliasTy { def_id , .. }) if self . can_define_opaque_ty (def_id) && ! ty . has_escaping_bound_vars () => { let def_span = self . tcx . def_span (def_id) ; let span = if span . contains (def_span) { def_span } else { span } ; let ty_var = self . next_ty_var (span) ; obligations . extend (self . handle_opaque_type (ty , ty_var , span , param_env) . unwrap () . into_iter () . map (| goal | { Obligation :: new (self . tcx , ObligationCause :: new (span , body_id , traits :: ObligationCauseCode :: OpaqueReturnType (None) ,) , goal . param_env , goal . predicate ,) }) ,) ; ty_var } _ => ty , } , }) ; InferOk { value , obligations } } pub fn handle_opaque_type (& self , a : Ty < 'tcx > , b : Ty < 'tcx > , span : Span , param_env : ty :: ParamEnv < 'tcx > ,) -> Result < Vec < Goal < 'tcx , ty :: Predicate < 'tcx > > > , TypeError < 'tcx > > { debug_assert ! (! self . next_trait_solver ()) ; let process = | a : Ty < 'tcx > , b : Ty < 'tcx > | match * a . kind () { ty :: Alias (ty :: Opaque , ty :: AliasTy { def_id , args , .. }) if def_id . is_local () => { let def_id = def_id . expect_local () ; if let ty :: TypingMode :: Coherence = self . typing_mode () { return Some (self . register_hidden_type (OpaqueTypeKey { def_id , args } , span , param_env , b ,)) ; } if ! self . can_define_opaque_ty (def_id) { return None ; } if let ty :: Alias (ty :: Opaque , ty :: AliasTy { def_id : b_def_id , .. }) = * b . kind () { if self . can_define_opaque_ty (b_def_id) && matches ! (self . tcx . opaque_ty_origin (b_def_id) , hir :: OpaqueTyOrigin :: TyAlias { .. }) { self . dcx () . emit_err (OpaqueHiddenTypeDiag { span , hidden_type : self . tcx . def_span (b_def_id) , opaque_type : self . tcx . def_span (def_id) , }) ; } } Some (self . register_hidden_type (OpaqueTypeKey { def_id , args } , span , param_env , b)) } _ => None , } ; if let Some (res) = process (a , b) { res } else if let Some (res) = process (b , a) { res } else { let (a , b) = self . resolve_vars_if_possible ((a , b)) ; Err (TypeError :: Sorts (ExpectedFound :: new (a , b))) } } }}}
mkitem!{mkimpl!{impl < 'tcx > InferCtxt < 'tcx > { #[instrument (skip (self) , level = "debug")] fn register_hidden_type (& self , opaque_type_key : OpaqueTypeKey < 'tcx > , span : Span , param_env : ty :: ParamEnv < 'tcx > , hidden_ty : Ty < 'tcx > ,) -> Result < Vec < Goal < 'tcx , ty :: Predicate < 'tcx > > > , TypeError < 'tcx > > { let mut goals = Vec :: new () ; self . insert_hidden_type (opaque_type_key , span , param_env , hidden_ty , & mut goals) ? ; self . add_item_bounds_for_hidden_type (opaque_type_key . def_id . to_def_id () , opaque_type_key . args , param_env , hidden_ty , & mut goals ,) ; Ok (goals) } #[doc = " Insert a hidden type into the opaque type storage, making sure"] #[doc = " it hasn't previously been defined. This does not emit any"] #[doc = " constraints and it's the responsibility of the caller to make"] #[doc = " sure that the item bounds of the opaque are checked."] pub fn register_hidden_type_in_storage (& self , opaque_type_key : OpaqueTypeKey < 'tcx > , hidden_ty : OpaqueHiddenType < 'tcx > ,) -> Option < Ty < 'tcx > > { self . inner . borrow_mut () . opaque_types () . register (opaque_type_key , hidden_ty) } #[doc = " Insert a hidden type into the opaque type storage, equating it"] #[doc = " with any previous entries if necessary."] #[doc = ""] #[doc = " This **does not** add the item bounds of the opaque as nested"] #[doc = " obligations. That is only necessary when normalizing the opaque"] #[doc = " itself, not when getting the opaque type constraints from"] #[doc = " somewhere else."] pub fn insert_hidden_type (& self , opaque_type_key : OpaqueTypeKey < 'tcx > , span : Span , param_env : ty :: ParamEnv < 'tcx > , hidden_ty : Ty < 'tcx > , goals : & mut Vec < Goal < 'tcx , ty :: Predicate < 'tcx > > > ,) -> Result < () , TypeError < 'tcx > > { let tcx = self . tcx ; match self . typing_mode () { ty :: TypingMode :: Coherence => { goals . push (Goal :: new (tcx , param_env , ty :: PredicateKind :: Ambiguous)) ; } ty :: TypingMode :: Analysis { .. } => { let prev = self . inner . borrow_mut () . opaque_types () . register (opaque_type_key , OpaqueHiddenType { ty : hidden_ty , span }) ; if let Some (prev) = prev { goals . extend (self . at (& ObligationCause :: dummy_with_span (span) , param_env) . eq (DefineOpaqueTypes :: Yes , prev , hidden_ty) ? . obligations . into_iter () . map (| obligation | obligation . as_goal ()) ,) ; } } ty :: TypingMode :: Borrowck { .. } => { let prev = self . inner . borrow_mut () . opaque_types () . register (opaque_type_key , OpaqueHiddenType { ty : hidden_ty , span }) ; let actual = prev . unwrap_or_else (| | { let actual = tcx . type_of_opaque_hir_typeck (opaque_type_key . def_id) . instantiate (self . tcx , opaque_type_key . args) ; let actual = ty :: fold_regions (tcx , actual , | re , _dbi | match re . kind () { ty :: ReErased => self . next_region_var (RegionVariableOrigin :: Misc (span)) , _ => re , }) ; actual }) ; goals . extend (self . at (& ObligationCause :: dummy_with_span (span) , param_env) . eq (DefineOpaqueTypes :: Yes , hidden_ty , actual) ? . obligations . into_iter () . map (| obligation | obligation . as_goal ()) ,) ; } mode @ (ty :: TypingMode :: PostBorrowckAnalysis { .. } | ty :: TypingMode :: PostAnalysis) => { bug ! ("insert hidden type in {mode:?}") } } Ok (()) } pub fn add_item_bounds_for_hidden_type (& self , def_id : DefId , args : ty :: GenericArgsRef < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , hidden_ty : Ty < 'tcx > , goals : & mut Vec < Goal < 'tcx , ty :: Predicate < 'tcx > > > ,) { let tcx = self . tcx ; goals . push (Goal :: new (tcx , param_env , ty :: ClauseKind :: WellFormed (hidden_ty . into ()))) ; let replace_opaques_in = | clause : ty :: Clause < 'tcx > , goals : & mut Vec < _ > | { clause . fold_with (& mut BottomUpFolder { tcx , ty_op : | ty | match * ty . kind () { ty :: Alias (ty :: Projection , projection_ty) if ! projection_ty . has_escaping_bound_vars () && ! tcx . is_impl_trait_in_trait (projection_ty . def_id) && ! self . next_trait_solver () => { let ty_var = self . next_ty_var (self . tcx . def_span (projection_ty . def_id)) ; goals . push (Goal :: new (self . tcx , param_env , ty :: PredicateKind :: Clause (ty :: ClauseKind :: Projection (ty :: ProjectionPredicate { projection_term : projection_ty . into () , term : ty_var . into () , } ,)) ,)) ; ty_var } ty :: Alias (ty :: Opaque , ty :: AliasTy { def_id : def_id2 , args : args2 , .. }) if def_id == def_id2 && args == args2 => { hidden_ty } _ => ty , } , lt_op : | lt | lt , ct_op : | ct | ct , }) } ; let item_bounds = tcx . explicit_item_bounds (def_id) ; for (predicate , _) in item_bounds . iter_instantiated_copied (tcx , args) { let predicate = replace_opaques_in (predicate , goals) ; debug ! (? predicate) ; goals . push (Goal :: new (self . tcx , param_env , predicate)) ; } if self . tcx . is_conditionally_const (def_id) { let item_bounds = tcx . explicit_implied_const_bounds (def_id) ; for (predicate , _) in item_bounds . iter_instantiated_copied (tcx , args) { let predicate = replace_opaques_in (predicate . to_host_effect_clause (self . tcx , ty :: BoundConstness :: Maybe) , goals ,) ; debug ! (? predicate) ; goals . push (Goal :: new (self . tcx , param_env , predicate)) ; } } } }}}