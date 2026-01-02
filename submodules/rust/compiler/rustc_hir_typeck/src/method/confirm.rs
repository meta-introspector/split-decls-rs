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
mkuse!{use std :: ops :: Deref ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: GenericArg ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_hir_analysis :: hir_ty_lowering :: generics :: { check_generic_arg_count_for_call , lower_generic_args , } ;}
mkuse!{use rustc_hir_analysis :: hir_ty_lowering :: { FeedConstTy , GenericArgsLowerer , HirTyLowerer , IsMethodCall , RegionInferReason , } ;}
mkuse!{use rustc_infer :: infer :: { BoundRegionConversionTime , DefineOpaqueTypes , InferOk , RegionVariableOrigin , } ;}
mkuse!{use rustc_lint :: builtin :: SUPERTRAIT_ITEM_SHADOWING_USAGE ;}
mkuse!{use rustc_middle :: traits :: ObligationCauseCode ;}
mkuse!{use rustc_middle :: ty :: adjustment :: { Adjust , Adjustment , AllowTwoPhase , AutoBorrow , AutoBorrowMutability , PointerCoercion , } ;}
mkuse!{use rustc_middle :: ty :: { self , GenericArgs , GenericArgsRef , GenericParamDefKind , Ty , TyCtxt , TypeFoldable , TypeVisitableExt , UserArgs , } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_span :: { DUMMY_SP , Span } ;}
mkuse!{use rustc_trait_selection :: traits ;}
mkuse!{use tracing :: debug ;}
mkuse!{use super :: { MethodCallee , probe } ;}
mkuse!{use crate :: errors :: { SupertraitItemShadowee , SupertraitItemShadower , SupertraitItemShadowing } ;}
mkuse!{use crate :: { FnCtxt , callee } ;}
mkitem!{mkstruct!{struct ConfirmContext < 'a , 'tcx > { fcx : & 'a FnCtxt < 'a , 'tcx > , span : Span , self_expr : & 'tcx hir :: Expr < 'tcx > , call_expr : & 'tcx hir :: Expr < 'tcx > , skip_record_for_diagnostics : bool , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Deref for ConfirmContext < 'a , 'tcx > { type Target = FnCtxt < 'a , 'tcx > ; fn deref (& self) -> & Self :: Target { self . fcx } }}}
mkitem!{mkstruct!{#[derive (Debug)] pub (crate) struct ConfirmResult < 'tcx > { pub callee : MethodCallee < 'tcx > , pub illegal_sized_bound : Option < Span > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > FnCtxt < 'a , 'tcx > { pub (crate) fn confirm_method (& self , span : Span , self_expr : & 'tcx hir :: Expr < 'tcx > , call_expr : & 'tcx hir :: Expr < 'tcx > , unadjusted_self_ty : Ty < 'tcx > , pick : & probe :: Pick < 'tcx > , segment : & 'tcx hir :: PathSegment < 'tcx > ,) -> ConfirmResult < 'tcx > { debug ! ("confirm(unadjusted_self_ty={:?}, pick={:?}, generic_args={:?})" , unadjusted_self_ty , pick , segment . args ,) ; let mut confirm_cx = ConfirmContext :: new (self , span , self_expr , call_expr) ; confirm_cx . confirm (unadjusted_self_ty , pick , segment) } pub (crate) fn confirm_method_for_diagnostic (& self , span : Span , self_expr : & 'tcx hir :: Expr < 'tcx > , call_expr : & 'tcx hir :: Expr < 'tcx > , unadjusted_self_ty : Ty < 'tcx > , pick : & probe :: Pick < 'tcx > , segment : & hir :: PathSegment < 'tcx > ,) -> ConfirmResult < 'tcx > { let mut confirm_cx = ConfirmContext :: new (self , span , self_expr , call_expr) ; confirm_cx . skip_record_for_diagnostics = true ; confirm_cx . confirm (unadjusted_self_ty , pick , segment) } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > ConfirmContext < 'a , 'tcx > { fn new (fcx : & 'a FnCtxt < 'a , 'tcx > , span : Span , self_expr : & 'tcx hir :: Expr < 'tcx > , call_expr : & 'tcx hir :: Expr < 'tcx > ,) -> ConfirmContext < 'a , 'tcx > { ConfirmContext { fcx , span , self_expr , call_expr , skip_record_for_diagnostics : false } } fn confirm (& mut self , unadjusted_self_ty : Ty < 'tcx > , pick : & probe :: Pick < 'tcx > , segment : & hir :: PathSegment < 'tcx > ,) -> ConfirmResult < 'tcx > { let self_ty = self . adjust_self_ty (unadjusted_self_ty , pick) ; let rcvr_args = self . fresh_receiver_args (self_ty , pick) ; let all_args = self . instantiate_method_args (pick , segment , rcvr_args) ; debug ! ("rcvr_args={rcvr_args:?}, all_args={all_args:?}") ; let (method_sig , method_predicates) = self . instantiate_method_sig (pick , all_args) ; let filler_args = rcvr_args . extend_to (self . tcx , pick . item . def_id , | def , _ | self . tcx . mk_param_from_def (def)) ; let illegal_sized_bound = self . predicates_require_illegal_sized_bound (self . tcx . predicates_of (pick . item . def_id) . instantiate (self . tcx , filler_args) ,) ; let method_sig_rcvr = self . normalize (self . span , method_sig . inputs () [0]) ; debug ! ("confirm: self_ty={:?} method_sig_rcvr={:?} method_sig={:?} method_predicates={:?}" , self_ty , method_sig_rcvr , method_sig , method_predicates) ; self . unify_receivers (self_ty , method_sig_rcvr , pick) ; let (method_sig , method_predicates) = self . normalize (self . span , (method_sig , method_predicates)) ; self . check_for_illegal_method_calls (pick) ; self . lint_shadowed_supertrait_items (pick , segment) ; if illegal_sized_bound . is_none () { self . add_obligations (method_sig , all_args , method_predicates , pick . item . def_id) ; } let callee = MethodCallee { def_id : pick . item . def_id , args : all_args , sig : method_sig } ; ConfirmResult { callee , illegal_sized_bound } } fn adjust_self_ty (& mut self , unadjusted_self_ty : Ty < 'tcx > , pick : & probe :: Pick < 'tcx > ,) -> Ty < 'tcx > { let mut autoderef = self . autoderef (self . call_expr . span , unadjusted_self_ty) ; let Some ((ty , n)) = autoderef . nth (pick . autoderefs) else { return Ty :: new_error_with_message (self . tcx , DUMMY_SP , format ! ("failed autoderef {}" , pick . autoderefs) ,) ; } ; assert_eq ! (n , pick . autoderefs) ; let mut adjustments = self . adjust_steps (& autoderef) ; let mut target = self . structurally_resolve_type (autoderef . span () , ty) ; match pick . autoref_or_ptr_adjustment { Some (probe :: AutorefOrPtrAdjustment :: Autoref { mutbl , unsize }) => { let region = self . next_region_var (RegionVariableOrigin :: Autoref (self . span)) ; let base_ty = target ; target = Ty :: new_ref (self . tcx , region , target , mutbl) ; let mutbl = AutoBorrowMutability :: new (mutbl , AllowTwoPhase :: Yes) ; adjustments . push (Adjustment { kind : Adjust :: Borrow (AutoBorrow :: Ref (mutbl)) , target }) ; if unsize { let unsized_ty = if let ty :: Array (elem_ty , _) = base_ty . kind () { Ty :: new_slice (self . tcx , * elem_ty) } else { bug ! ("AutorefOrPtrAdjustment's unsize flag should only be set for array ty, found {}" , base_ty) } ; target = Ty :: new_ref (self . tcx , region , unsized_ty , mutbl . into ()) ; adjustments . push (Adjustment { kind : Adjust :: Pointer (PointerCoercion :: Unsize) , target , }) ; } } Some (probe :: AutorefOrPtrAdjustment :: ToConstPtr) => { target = match target . kind () { & ty :: RawPtr (ty , mutbl) => { assert ! (mutbl . is_mut ()) ; Ty :: new_imm_ptr (self . tcx , ty) } other => panic ! ("Cannot adjust receiver type {other:?} to const ptr") , } ; adjustments . push (Adjustment { kind : Adjust :: Pointer (PointerCoercion :: MutToConstPointer) , target , }) ; } Some (probe :: AutorefOrPtrAdjustment :: ReborrowPin (mutbl)) => { let region = self . next_region_var (RegionVariableOrigin :: Autoref (self . span)) ; target = match target . kind () { ty :: Adt (pin , args) if self . tcx . is_lang_item (pin . did () , hir :: LangItem :: Pin) => { let inner_ty = match args [0] . expect_ty () . kind () { ty :: Ref (_ , ty , _) => * ty , _ => bug ! ("Expected a reference type for argument to Pin") , } ; Ty :: new_pinned_ref (self . tcx , region , inner_ty , mutbl) } _ => bug ! ("Cannot adjust receiver type for reborrowing pin of {target:?}") , } ; adjustments . push (Adjustment { kind : Adjust :: ReborrowPin (mutbl) , target }) ; } None => { } } self . register_predicates (autoderef . into_obligations ()) ; if ! self . skip_record_for_diagnostics { self . apply_adjustments (self . self_expr , adjustments) ; } target } #[doc = " Returns a set of generic parameters for the method *receiver* where all type and region"] #[doc = " parameters are instantiated with fresh variables. This generic parameters does not include any"] #[doc = " parameters declared on the method itself."] #[doc = ""] #[doc = " Note that this generic parameters may include late-bound regions from the impl level. If so,"] #[doc = " these are instantiated later in the `instantiate_method_sig` routine."] fn fresh_receiver_args (& mut self , self_ty : Ty < 'tcx > , pick : & probe :: Pick < 'tcx > ,) -> GenericArgsRef < 'tcx > { match pick . kind { probe :: InherentImplPick => { let impl_def_id = pick . item . container_id (self . tcx) ; assert ! (self . tcx . impl_trait_ref (impl_def_id) . is_none () , "impl {impl_def_id:?} is not an inherent impl") ; self . fresh_args_for_item (self . span , impl_def_id) } probe :: ObjectPick => { let trait_def_id = pick . item . container_id (self . tcx) ; if ! self . tcx . is_dyn_compatible (trait_def_id) { return ty :: GenericArgs :: extend_with_error (self . tcx , trait_def_id , & []) ; } if self_ty . references_error () { return ty :: GenericArgs :: extend_with_error (self . tcx , trait_def_id , & []) ; } self . extract_existential_trait_ref (self_ty , | this , object_ty , principal | { let original_poly_trait_ref = principal . with_self_ty (this . tcx , object_ty) ; let upcast_poly_trait_ref = this . upcast (original_poly_trait_ref , trait_def_id) ; let upcast_trait_ref = this . instantiate_binder_with_fresh_vars (upcast_poly_trait_ref) ; debug ! ("original_poly_trait_ref={:?} upcast_trait_ref={:?} target_trait={:?}" , original_poly_trait_ref , upcast_trait_ref , trait_def_id) ; upcast_trait_ref . args }) } probe :: TraitPick => { let trait_def_id = pick . item . container_id (self . tcx) ; self . fresh_args_for_item (self . span , trait_def_id) } probe :: WhereClausePick (poly_trait_ref) => { self . instantiate_binder_with_fresh_vars (poly_trait_ref) . args } } } fn extract_existential_trait_ref < R , F > (& mut self , self_ty : Ty < 'tcx > , mut closure : F) -> R where F : FnMut (& mut ConfirmContext < 'a , 'tcx > , Ty < 'tcx > , ty :: PolyExistentialTraitRef < 'tcx >) -> R , { let mut autoderef = self . fcx . autoderef (self . span , self_ty) ; if self . tcx . features () . arbitrary_self_types () || self . tcx . features () . arbitrary_self_types_pointers () { autoderef = autoderef . use_receiver_trait () ; } autoderef . include_raw_pointers () . find_map (| (ty , _) | match ty . kind () { ty :: Dynamic (data , ..) => Some (closure (self , ty , data . principal () . unwrap_or_else (| | { span_bug ! (self . span , "calling trait method on empty object?") }) ,)) , _ => None , }) . unwrap_or_else (| | { span_bug ! (self . span , "self-type `{}` for ObjectPick never dereferenced to an object" , self_ty) }) } fn instantiate_method_args (& mut self , pick : & probe :: Pick < 'tcx > , seg : & hir :: PathSegment < 'tcx > , parent_args : GenericArgsRef < 'tcx > ,) -> GenericArgsRef < 'tcx > { let generics = self . tcx . generics_of (pick . item . def_id) ; let arg_count_correct = check_generic_arg_count_for_call (self . fcx , pick . item . def_id , generics , seg , IsMethodCall :: Yes ,) ; assert_eq ! (generics . parent_count , parent_args . len ()) ; struct GenericArgsCtxt < 'a , 'tcx > { cfcx : & 'a ConfirmContext < 'a , 'tcx > , pick : & 'a probe :: Pick < 'tcx > , seg : & 'a hir :: PathSegment < 'tcx > , } impl < 'a , 'tcx > GenericArgsLowerer < 'a , 'tcx > for GenericArgsCtxt < 'a , 'tcx > { fn args_for_def_id (& mut self , def_id : DefId ,) -> (Option < & 'a hir :: GenericArgs < 'tcx > > , bool) { if def_id == self . pick . item . def_id { if let Some (data) = self . seg . args { return (Some (data) , false) ; } } (None , false) } fn provided_kind (& mut self , preceding_args : & [ty :: GenericArg < 'tcx >] , param : & ty :: GenericParamDef , arg : & GenericArg < 'tcx > ,) -> ty :: GenericArg < 'tcx > { match (& param . kind , arg) { (GenericParamDefKind :: Lifetime , GenericArg :: Lifetime (lt)) => self . cfcx . fcx . lowerer () . lower_lifetime (lt , RegionInferReason :: Param (param)) . into () , (GenericParamDefKind :: Type { .. } , GenericArg :: Type (ty)) => { self . cfcx . lower_ty (ty . as_unambig_ty ()) . raw . into () } (GenericParamDefKind :: Type { .. } , GenericArg :: Infer (inf)) => { self . cfcx . lower_ty (& inf . to_ty ()) . raw . into () } (GenericParamDefKind :: Const { .. } , GenericArg :: Const (ct)) => self . cfcx . lower_const_arg (ct . as_unambig_ct () , FeedConstTy :: Param (param . def_id , preceding_args) ,) . into () , (GenericParamDefKind :: Const { .. } , GenericArg :: Infer (inf)) => { self . cfcx . ct_infer (Some (param) , inf . span) . into () } (kind , arg) => { bug ! ("mismatched method arg kind {kind:?} in turbofish: {arg:?}") } } } fn inferred_kind (& mut self , _preceding_args : & [ty :: GenericArg < 'tcx >] , param : & ty :: GenericParamDef , _infer_args : bool ,) -> ty :: GenericArg < 'tcx > { self . cfcx . var_for_def (self . cfcx . span , param) } } let args = lower_generic_args (self . fcx , pick . item . def_id , parent_args , false , None , & arg_count_correct , & mut GenericArgsCtxt { cfcx : self , pick , seg } ,) ; if ! args . is_empty () && ! generics . is_own_empty () { let user_type_annotation = self . probe (| _ | { let user_args = UserArgs { args : GenericArgs :: for_item (self . tcx , pick . item . def_id , | param , _ | { let i = param . index as usize ; if i < generics . parent_count { self . fcx . var_for_def (DUMMY_SP , param) } else { args [i] } }) , user_self_ty : None , } ; self . fcx . canonicalize_user_type_annotation (ty :: UserType :: new (ty :: UserTypeKind :: TypeOf (pick . item . def_id , user_args) ,)) }) ; debug ! ("instantiate_method_args: user_type_annotation={:?}" , user_type_annotation) ; if ! self . skip_record_for_diagnostics { self . fcx . write_user_type_annotation (self . call_expr . hir_id , user_type_annotation) ; } } self . normalize (self . span , args) } fn unify_receivers (& mut self , self_ty : Ty < 'tcx > , method_self_ty : Ty < 'tcx > , pick : & probe :: Pick < 'tcx > ,) { debug ! ("unify_receivers: self_ty={:?} method_self_ty={:?} span={:?} pick={:?}" , self_ty , method_self_ty , self . span , pick) ; let cause = self . cause (self . self_expr . span , ObligationCauseCode :: Misc) ; match self . at (& cause , self . param_env) . sup (DefineOpaqueTypes :: Yes , method_self_ty , self_ty) { Ok (InferOk { obligations , value : () }) => { self . register_predicates (obligations) ; } Err (terr) => { if self . tcx . features () . arbitrary_self_types () { self . err_ctxt () . report_mismatched_types (& cause , self . param_env , method_self_ty , self_ty , terr ,) . emit () ; } else { self . dcx () . span_delayed_bug (cause . span , format ! ("{self_ty} was a subtype of {method_self_ty} but now is not?") ,) ; } } } } fn instantiate_method_sig (& mut self , pick : & probe :: Pick < 'tcx > , all_args : GenericArgsRef < 'tcx > ,) -> (ty :: FnSig < 'tcx > , ty :: InstantiatedPredicates < 'tcx >) { debug ! ("instantiate_method_sig(pick={:?}, all_args={:?})" , pick , all_args) ; let def_id = pick . item . def_id ; let method_predicates = self . tcx . predicates_of (def_id) . instantiate (self . tcx , all_args) ; debug ! ("method_predicates after instantiation = {:?}" , method_predicates) ; let sig = self . tcx . fn_sig (def_id) . instantiate (self . tcx , all_args) ; debug ! ("type scheme instantiated, sig={:?}" , sig) ; let sig = self . instantiate_binder_with_fresh_vars (sig) ; debug ! ("late-bound lifetimes from method instantiated, sig={:?}" , sig) ; (sig , method_predicates) } fn add_obligations (& mut self , sig : ty :: FnSig < 'tcx > , all_args : GenericArgsRef < 'tcx > , method_predicates : ty :: InstantiatedPredicates < 'tcx > , def_id : DefId ,) { debug ! ("add_obligations: sig={:?} all_args={:?} method_predicates={:?} def_id={:?}" , sig , all_args , method_predicates , def_id) ; for obligation in traits :: predicates_for_generics (| idx , span | { let code = ObligationCauseCode :: WhereClauseInExpr (def_id , span , self . call_expr . hir_id , idx ,) ; self . cause (self . span , code) } , self . param_env , method_predicates ,) { self . register_predicate (obligation) ; } self . add_wf_bounds (all_args , self . call_expr . span) ; for ty in sig . inputs_and_output { self . register_wf_obligation (ty . into () , self . span , ObligationCauseCode :: WellFormed (None) ,) ; } } fn predicates_require_illegal_sized_bound (& self , predicates : ty :: InstantiatedPredicates < 'tcx > ,) -> Option < Span > { let sized_def_id = self . tcx . lang_items () . sized_trait () ? ; traits :: elaborate (self . tcx , predicates . predicates . iter () . copied ()) . filter_map (| pred | match pred . kind () . skip_binder () { ty :: ClauseKind :: Trait (trait_pred) if trait_pred . def_id () == sized_def_id => { let span = predicates . iter () . find_map (| (p , span) | if p == pred { Some (span) } else { None }) . unwrap_or (DUMMY_SP) ; Some ((trait_pred , span)) } _ => None , }) . find_map (| (trait_pred , span) | match trait_pred . self_ty () . kind () { ty :: Dynamic (..) => Some (span) , _ => None , }) } fn check_for_illegal_method_calls (& self , pick : & probe :: Pick < '_ >) { if let Some (trait_def_id) = pick . item . trait_container (self . tcx) && let Err (e) = callee :: check_legal_trait_for_method_call (self . tcx , self . span , Some (self . self_expr . span) , self . call_expr . span , trait_def_id , self . body_id . to_def_id () ,) { self . set_tainted_by_errors (e) ; } } fn lint_shadowed_supertrait_items (& self , pick : & probe :: Pick < '_ > , segment : & hir :: PathSegment < 'tcx > ,) { if pick . shadowed_candidates . is_empty () { return ; } let shadower_span = self . tcx . def_span (pick . item . def_id) ; let subtrait = self . tcx . item_name (pick . item . trait_container (self . tcx) . unwrap ()) ; let shadower = SupertraitItemShadower { span : shadower_span , subtrait } ; let shadowee = if let [shadowee] = & pick . shadowed_candidates [..] { let shadowee_span = self . tcx . def_span (shadowee . def_id) ; let supertrait = self . tcx . item_name (shadowee . trait_container (self . tcx) . unwrap ()) ; SupertraitItemShadowee :: Labeled { span : shadowee_span , supertrait } } else { let (traits , spans) : (Vec < _ > , Vec < _ >) = pick . shadowed_candidates . iter () . map (| item | { (self . tcx . item_name (item . trait_container (self . tcx) . unwrap ()) , self . tcx . def_span (item . def_id) ,) }) . unzip () ; SupertraitItemShadowee :: Several { traits : traits . into () , spans : spans . into () } } ; self . tcx . emit_node_span_lint (SUPERTRAIT_ITEM_SHADOWING_USAGE , segment . hir_id , segment . ident . span , SupertraitItemShadowing { shadower , shadowee , item : segment . ident . name , subtrait } ,) ; } fn upcast (& mut self , source_trait_ref : ty :: PolyTraitRef < 'tcx > , target_trait_def_id : DefId ,) -> ty :: PolyTraitRef < 'tcx > { let upcast_trait_refs = traits :: upcast_choices (self . tcx , source_trait_ref , target_trait_def_id) ; if let & [upcast_trait_ref] = upcast_trait_refs . as_slice () { upcast_trait_ref } else { self . dcx () . span_delayed_bug (self . span , format ! ("cannot uniquely upcast `{:?}` to `{:?}`: `{:?}`" , source_trait_ref , target_trait_def_id , upcast_trait_refs) ,) ; ty :: Binder :: dummy (ty :: TraitRef :: new_from_args (self . tcx , target_trait_def_id , ty :: GenericArgs :: extend_with_error (self . tcx , target_trait_def_id , & []) ,)) } } fn instantiate_binder_with_fresh_vars < T > (& self , value : ty :: Binder < 'tcx , T >) -> T where T : TypeFoldable < TyCtxt < 'tcx > > + Copy , { self . fcx . instantiate_binder_with_fresh_vars (self . span , BoundRegionConversionTime :: FnCall , value ,) } }}}