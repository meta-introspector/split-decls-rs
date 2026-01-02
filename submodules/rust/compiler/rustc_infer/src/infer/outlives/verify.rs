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
mkuse!{use std :: assert_matches :: assert_matches ;}
mkuse!{use rustc_middle :: ty :: outlives :: { Component , compute_alias_components_recursive } ;}
mkuse!{use rustc_middle :: ty :: { self , OutlivesPredicate , Ty , TyCtxt } ;}
mkuse!{use smallvec :: smallvec ;}
mkuse!{use tracing :: { debug , instrument , trace } ;}
mkuse!{use crate :: infer :: outlives :: env :: RegionBoundPairs ;}
mkuse!{use crate :: infer :: region_constraints :: VerifyIfEq ;}
mkuse!{use crate :: infer :: { GenericKind , VerifyBound } ;}
mkitem!{mkstruct!{#[doc = " The `TypeOutlives` struct has the job of \"lowering\" a `T: 'a`"] #[doc = " obligation into a series of `'a: 'b` constraints and \"verifys\", as"] #[doc = " described on the module comment. The final constraints are emitted"] #[doc = " via a \"delegate\" of type `D` -- this is usually the `infcx`, which"] #[doc = " accrues them into the `region_obligations` code, but for NLL we"] #[doc = " use something else."] pub (crate) struct VerifyBoundCx < 'cx , 'tcx > { tcx : TyCtxt < 'tcx > , region_bound_pairs : & 'cx RegionBoundPairs < 'tcx > , #[doc = " During borrowck, if there are no outlives bounds on a generic"] #[doc = " parameter `T`, we assume that `T: 'in_fn_body` holds."] #[doc = ""] #[doc = " Outside of borrowck the only way to prove `T: '?0` is by"] #[doc = " setting  `'?0` to `'empty`."] implicit_region_bound : Option < ty :: Region < 'tcx > > , caller_bounds : & 'cx [ty :: PolyTypeOutlivesPredicate < 'tcx >] , }}}
mkitem!{mkimpl!{impl < 'cx , 'tcx > VerifyBoundCx < 'cx , 'tcx > { pub (crate) fn new (tcx : TyCtxt < 'tcx > , region_bound_pairs : & 'cx RegionBoundPairs < 'tcx > , implicit_region_bound : Option < ty :: Region < 'tcx > > , caller_bounds : & 'cx [ty :: PolyTypeOutlivesPredicate < 'tcx >] ,) -> Self { Self { tcx , region_bound_pairs , implicit_region_bound , caller_bounds } } #[instrument (level = "debug" , skip (self))] pub (crate) fn param_or_placeholder_bound (& self , ty : Ty < 'tcx >) -> VerifyBound < 'tcx > { let declared_bounds_from_env = self . declared_generic_bounds_from_env (ty) ; debug ! (? declared_bounds_from_env) ; let mut param_bounds = vec ! [] ; for declared_bound in declared_bounds_from_env { let bound_region = declared_bound . map_bound (| outlives | outlives . 1) ; if let Some (region) = bound_region . no_bound_vars () { param_bounds . push (VerifyBound :: OutlivedBy (region)) ; } else { debug ! ("found that {ty:?} outlives any lifetime, returning empty vector") ; return VerifyBound :: AllBounds (vec ! []) ; } } if let Some (r) = self . implicit_region_bound { debug ! ("adding implicit region bound of {r:?}") ; param_bounds . push (VerifyBound :: OutlivedBy (r)) ; } if param_bounds . is_empty () { VerifyBound :: IsEmpty } else if param_bounds . len () == 1 { param_bounds . pop () . unwrap () } else { VerifyBound :: AnyBound (param_bounds) } } #[doc = " Given a projection like `T::Item`, searches the environment"] #[doc = " for where-clauses like `T::Item: 'a`. Returns the set of"] #[doc = " regions `'a` that it finds."] #[doc = ""] #[doc = " This is an \"approximate\" check -- it may not find all"] #[doc = " applicable bounds, and not all the bounds it returns can be"] #[doc = " relied upon. In particular, this check ignores region"] #[doc = " identity. So, for example, if we have `<T as"] #[doc = " Trait<'0>>::Item` where `'0` is a region variable, and the"] #[doc = " user has `<T as Trait<'a>>::Item: 'b` in the environment, then"] #[doc = " the clause from the environment only applies if `'0 = 'a`,"] #[doc = " which we don't know yet. But we would still include `'b` in"] #[doc = " this list."] pub (crate) fn approx_declared_bounds_from_env (& self , alias_ty : ty :: AliasTy < 'tcx > ,) -> Vec < ty :: PolyTypeOutlivesPredicate < 'tcx > > { let erased_alias_ty = self . tcx . erase_and_anonymize_regions (alias_ty . to_ty (self . tcx)) ; self . declared_generic_bounds_from_env_for_erased_ty (erased_alias_ty) } #[instrument (level = "debug" , skip (self))] pub (crate) fn alias_bound (& self , alias_ty : ty :: AliasTy < 'tcx >) -> VerifyBound < 'tcx > { let env_bounds = self . approx_declared_bounds_from_env (alias_ty) . into_iter () . map (| binder | { if let Some (ty :: OutlivesPredicate (ty , r)) = binder . no_bound_vars () && let ty :: Alias (_ , alias_ty_from_bound) = * ty . kind () && alias_ty_from_bound == alias_ty { VerifyBound :: OutlivedBy (r) } else { let verify_if_eq_b = binder . map_bound (| ty :: OutlivesPredicate (ty , bound) | VerifyIfEq { ty , bound }) ; VerifyBound :: IfEq (verify_if_eq_b) } }) ; let definition_bounds = self . declared_bounds_from_definition (alias_ty) . map (| r | VerifyBound :: OutlivedBy (r)) ; let recursive_bound = { let mut components = smallvec ! [] ; let kind = alias_ty . kind (self . tcx) ; compute_alias_components_recursive (self . tcx , kind , alias_ty , & mut components) ; self . bound_from_components (& components) } ; VerifyBound :: AnyBound (env_bounds . chain (definition_bounds) . collect ()) . or (recursive_bound) } fn bound_from_components (& self , components : & [Component < TyCtxt < 'tcx > >]) -> VerifyBound < 'tcx > { let mut bounds = components . iter () . map (| component | self . bound_from_single_component (component)) . filter (| bound | ! bound . must_hold ()) ; match (bounds . next () , bounds . next ()) { (Some (first) , None) => first , (first , second) => { VerifyBound :: AllBounds (first . into_iter () . chain (second) . chain (bounds) . collect ()) } } } fn bound_from_single_component (& self , component : & Component < TyCtxt < 'tcx > > ,) -> VerifyBound < 'tcx > { match * component { Component :: Region (lt) => VerifyBound :: OutlivedBy (lt) , Component :: Param (param_ty) => self . param_or_placeholder_bound (param_ty . to_ty (self . tcx)) , Component :: Placeholder (placeholder_ty) => { self . param_or_placeholder_bound (Ty :: new_placeholder (self . tcx , placeholder_ty)) } Component :: Alias (alias_ty) => self . alias_bound (alias_ty) , Component :: EscapingAlias (ref components) => self . bound_from_components (components) , Component :: UnresolvedInferenceVariable (v) => { self . tcx . dcx () . delayed_bug (format ! ("unresolved inference variable in outlives: {v:?}")) ; VerifyBound :: AnyBound (vec ! []) } } } #[doc = " Searches the environment for where-clauses like `G: 'a` where"] #[doc = " `G` is either some type parameter `T` or a projection like"] #[doc = " `T::Item`. Returns a vector of the `'a` bounds it can find."] #[doc = ""] #[doc = " This is a conservative check -- it may not find all applicable"] #[doc = " bounds, but all the bounds it returns can be relied upon."] fn declared_generic_bounds_from_env (& self , generic_ty : Ty < 'tcx > ,) -> Vec < ty :: PolyTypeOutlivesPredicate < 'tcx > > { assert_matches ! (generic_ty . kind () , ty :: Param (_) | ty :: Placeholder (_)) ; self . declared_generic_bounds_from_env_for_erased_ty (generic_ty) } #[doc = " Searches the environment to find all bounds that apply to `erased_ty`."] #[doc = " Obviously these must be approximate -- they are in fact both *over* and"] #[doc = " and *under* approximated:"] #[doc = ""] #[doc = " * Over-approximated because we don't consider equality of regions."] #[doc = " * Under-approximated because we look for syntactic equality and so for complex types"] #[doc = "   like `<T as Foo<fn(&u32, &u32)>>::Item` or whatever we may fail to figure out"] #[doc = "   all the subtleties."] #[doc = ""] #[doc = " In some cases, such as when `erased_ty` represents a `ty::Param`, however,"] #[doc = " the result is precise."] #[instrument (level = "debug" , skip (self))] fn declared_generic_bounds_from_env_for_erased_ty (& self , erased_ty : Ty < 'tcx > ,) -> Vec < ty :: PolyTypeOutlivesPredicate < 'tcx > > { let tcx = self . tcx ; let mut bounds = vec ! [] ; bounds . extend (self . caller_bounds . iter () . copied () . filter (move | outlives_predicate | { super :: test_type_match :: can_match_erased_ty (tcx , * outlives_predicate , erased_ty) })) ; bounds . extend (self . region_bound_pairs . iter () . filter_map (| & OutlivesPredicate (p , r) | { debug ! ("declared_generic_bounds_from_env_for_erased_ty: region_bound_pair = {:?}" , (r , p)) ; match (& p , erased_ty . kind ()) { (GenericKind :: Param (p1) , ty :: Param (p2)) if p1 == p2 => { } (GenericKind :: Placeholder (p1) , ty :: Placeholder (p2)) if p1 == p2 => { } (GenericKind :: Alias (a1) , ty :: Alias (_ , a2)) if a1 . def_id == a2 . def_id => { } _ => return None , } let p_ty = p . to_ty (tcx) ; let erased_p_ty = self . tcx . erase_and_anonymize_regions (p_ty) ; (erased_p_ty == erased_ty) . then_some (ty :: Binder :: dummy (ty :: OutlivesPredicate (p_ty , r))) })) ; bounds } #[doc = " Given a projection like `<T as Foo<'x>>::Bar`, returns any bounds"] #[doc = " declared in the trait definition. For example, if the trait were"] #[doc = ""] #[doc = " ```rust"] #[doc = " trait Foo<'a> {"] #[doc = "     type Bar: 'a;"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " If we were given the `DefId` of `Foo::Bar`, we would return"] #[doc = " `'a`. You could then apply the instantiations from the"] #[doc = " projection to convert this into your namespace. This also"] #[doc = " works if the user writes `where <Self as Foo<'a>>::Bar: 'a` on"] #[doc = " the trait. In fact, it works by searching for just such a"] #[doc = " where-clause."] #[doc = ""] #[doc = " It will not, however, work for higher-ranked bounds like:"] #[doc = ""] #[doc = " ```ignore(this does compile today, previously was marked as `compile_fail,E0311`)"] #[doc = " trait Foo<'a, 'b>"] #[doc = " where for<'x> <Self as Foo<'x, 'b>>::Bar: 'x"] #[doc = " {"] #[doc = "     type Bar;"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc = " This is for simplicity, and because we are not really smart"] #[doc = " enough to cope with such bounds anywhere."] pub (crate) fn declared_bounds_from_definition (& self , alias_ty : ty :: AliasTy < 'tcx > ,) -> impl Iterator < Item = ty :: Region < 'tcx > > { let tcx = self . tcx ; let bounds = tcx . item_self_bounds (alias_ty . def_id) ; trace ! ("{:#?}" , bounds . skip_binder ()) ; bounds . iter_instantiated (tcx , alias_ty . args) . filter_map (| p | p . as_type_outlives_clause ()) . filter_map (| p | p . no_bound_vars ()) . map (| OutlivesPredicate (_ , r) | r) } }}}