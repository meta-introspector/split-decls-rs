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
mkuse!{use rustc_middle :: traits :: solve :: Goal ;}
mkuse!{use rustc_middle :: ty :: relate :: combine :: { super_combine_consts , super_combine_tys } ;}
mkuse!{use rustc_middle :: ty :: relate :: { Relate , RelateResult , TypeRelation , relate_args_invariantly , relate_args_with_variances , } ;}
mkuse!{use rustc_middle :: ty :: { self , DelayedSet , Ty , TyCtxt , TyVar } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use crate :: infer :: BoundRegionConversionTime :: HigherRankedType ;}
mkuse!{use crate :: infer :: relate :: { PredicateEmittingRelation , StructurallyRelateAliases } ;}
mkuse!{use crate :: infer :: { DefineOpaqueTypes , InferCtxt , SubregionOrigin , TypeTrace } ;}
mkuse!{use crate :: traits :: { Obligation , PredicateObligations } ;}
mkitem!{mkstruct!{#[doc = " Enforce that `a` is equal to or a subtype of `b`."] pub (crate) struct TypeRelating < 'infcx , 'tcx > { infcx : & 'infcx InferCtxt < 'tcx > , trace : TypeTrace < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , define_opaque_types : DefineOpaqueTypes , ambient_variance : ty :: Variance , obligations : PredicateObligations < 'tcx > , #[doc = " The cache only tracks the `ambient_variance` as it's the"] #[doc = " only field which is mutable and which meaningfully changes"] #[doc = " the result when relating types."] #[doc = ""] #[doc = " The cache does not track whether the state of the"] #[doc = " `InferCtxt` has been changed or whether we've added any"] #[doc = " obligations to `self.goals`. Whether a goal is added"] #[doc = " once or multiple times is not really meaningful."] #[doc = ""] #[doc = " Changes in the inference state may delay some type inference to"] #[doc = " the next fulfillment loop. Given that this loop is already"] #[doc = " necessary, this is also not a meaningful change. Consider"] #[doc = " the following three relations:"] #[doc = " ```text"] #[doc = " Vec<?0> sub Vec<?1>"] #[doc = " ?0 eq u32"] #[doc = " Vec<?0> sub Vec<?1>"] #[doc = " ```"] #[doc = " Without a cache, the second `Vec<?0> sub Vec<?1>` would eagerly"] #[doc = " constrain `?1` to `u32`. When using the cache entry from the"] #[doc = " first time we've related these types, this only happens when"] #[doc = " later proving the `Subtype(?0, ?1)` goal from the first relation."] cache : DelayedSet < (ty :: Variance , Ty < 'tcx > , Ty < 'tcx >) > , }}}
mkitem!{mkimpl!{impl < 'infcx , 'tcx > TypeRelating < 'infcx , 'tcx > { pub (crate) fn new (infcx : & 'infcx InferCtxt < 'tcx > , trace : TypeTrace < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , define_opaque_types : DefineOpaqueTypes , ambient_variance : ty :: Variance ,) -> TypeRelating < 'infcx , 'tcx > { assert ! (! infcx . next_trait_solver) ; TypeRelating { infcx , trace , param_env , define_opaque_types , ambient_variance , obligations : PredicateObligations :: new () , cache : Default :: default () , } } pub (crate) fn into_obligations (self) -> PredicateObligations < 'tcx > { self . obligations } }}}
mkitem!{mkimpl!{impl < 'tcx > TypeRelation < TyCtxt < 'tcx > > for TypeRelating < '_ , 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . infcx . tcx } fn relate_item_args (& mut self , item_def_id : rustc_hir :: def_id :: DefId , a_arg : ty :: GenericArgsRef < 'tcx > , b_arg : ty :: GenericArgsRef < 'tcx > ,) -> RelateResult < 'tcx , ty :: GenericArgsRef < 'tcx > > { if self . ambient_variance == ty :: Invariant { relate_args_invariantly (self , a_arg , b_arg) } else { let tcx = self . cx () ; let opt_variances = tcx . variances_of (item_def_id) ; relate_args_with_variances (self , item_def_id , opt_variances , a_arg , b_arg , false) } } fn relate_with_variance < T : Relate < TyCtxt < 'tcx > > > (& mut self , variance : ty :: Variance , _info : ty :: VarianceDiagInfo < TyCtxt < 'tcx > > , a : T , b : T ,) -> RelateResult < 'tcx , T > { let old_ambient_variance = self . ambient_variance ; self . ambient_variance = self . ambient_variance . xform (variance) ; debug ! (? self . ambient_variance , "new ambient variance") ; let r = if self . ambient_variance == ty :: Bivariant { Ok (a) } else { self . relate (a , b) } ; self . ambient_variance = old_ambient_variance ; r } #[instrument (skip (self) , level = "trace")] fn tys (& mut self , a : Ty < 'tcx > , b : Ty < 'tcx >) -> RelateResult < 'tcx , Ty < 'tcx > > { if a == b { return Ok (a) ; } let infcx = self . infcx ; let a = infcx . shallow_resolve (a) ; let b = infcx . shallow_resolve (b) ; if self . cache . contains (& (self . ambient_variance , a , b)) { return Ok (a) ; } match (a . kind () , b . kind ()) { (& ty :: Infer (TyVar (a_id)) , & ty :: Infer (TyVar (b_id))) => { match self . ambient_variance { ty :: Covariant => { self . obligations . push (Obligation :: new (self . cx () , self . trace . cause . clone () , self . param_env , ty :: Binder :: dummy (ty :: PredicateKind :: Subtype (ty :: SubtypePredicate { a_is_expected : true , a , b , })) ,)) ; } ty :: Contravariant => { self . obligations . push (Obligation :: new (self . cx () , self . trace . cause . clone () , self . param_env , ty :: Binder :: dummy (ty :: PredicateKind :: Subtype (ty :: SubtypePredicate { a_is_expected : false , a : b , b : a , })) ,)) ; } ty :: Invariant => { infcx . inner . borrow_mut () . type_variables () . equate (a_id , b_id) ; } ty :: Bivariant => { unreachable ! ("Expected bivariance to be handled in relate_with_variance") } } } (& ty :: Infer (TyVar (a_vid)) , _) => { infcx . instantiate_ty_var (self , true , a_vid , self . ambient_variance , b) ? ; } (_ , & ty :: Infer (TyVar (b_vid))) => { infcx . instantiate_ty_var (self , false , b_vid , self . ambient_variance . xform (ty :: Contravariant) , a ,) ? ; } (& ty :: Alias (ty :: Opaque , ty :: AliasTy { def_id : a_def_id , .. }) , & ty :: Alias (ty :: Opaque , ty :: AliasTy { def_id : b_def_id , .. }) ,) if a_def_id == b_def_id => { super_combine_tys (infcx , self , a , b) ? ; } (& ty :: Alias (ty :: Opaque , ty :: AliasTy { def_id , .. }) , _) | (_ , & ty :: Alias (ty :: Opaque , ty :: AliasTy { def_id , .. })) if self . define_opaque_types == DefineOpaqueTypes :: Yes && def_id . is_local () => { self . register_goals (infcx . handle_opaque_type (a , b , self . trace . cause . span , self . param_env () ,) ?) ; } _ => { super_combine_tys (infcx , self , a , b) ? ; } } assert ! (self . cache . insert ((self . ambient_variance , a , b))) ; Ok (a) } #[instrument (skip (self) , level = "trace")] fn regions (& mut self , a : ty :: Region < 'tcx > , b : ty :: Region < 'tcx > ,) -> RelateResult < 'tcx , ty :: Region < 'tcx > > { let origin = SubregionOrigin :: Subtype (Box :: new (self . trace . clone ())) ; match self . ambient_variance { ty :: Covariant => { self . infcx . inner . borrow_mut () . unwrap_region_constraints () . make_subregion (origin , b , a) ; } ty :: Contravariant => { self . infcx . inner . borrow_mut () . unwrap_region_constraints () . make_subregion (origin , a , b) ; } ty :: Invariant => { self . infcx . inner . borrow_mut () . unwrap_region_constraints () . make_eqregion (origin , a , b) ; } ty :: Bivariant => { unreachable ! ("Expected bivariance to be handled in relate_with_variance") } } Ok (a) } #[instrument (skip (self) , level = "trace")] fn consts (& mut self , a : ty :: Const < 'tcx > , b : ty :: Const < 'tcx > ,) -> RelateResult < 'tcx , ty :: Const < 'tcx > > { super_combine_consts (self . infcx , self , a , b) } fn binders < T > (& mut self , a : ty :: Binder < 'tcx , T > , b : ty :: Binder < 'tcx , T > ,) -> RelateResult < 'tcx , ty :: Binder < 'tcx , T > > where T : Relate < TyCtxt < 'tcx > > , { if a == b { } else if let Some (a) = a . no_bound_vars () && let Some (b) = b . no_bound_vars () { self . relate (a , b) ? ; } else { let span = self . trace . cause . span ; let infcx = self . infcx ; match self . ambient_variance { ty :: Covariant => { infcx . enter_forall (b , | b | { let a = infcx . instantiate_binder_with_fresh_vars (span , HigherRankedType , a) ; self . relate (a , b) }) ? ; } ty :: Contravariant => { infcx . enter_forall (a , | a | { let b = infcx . instantiate_binder_with_fresh_vars (span , HigherRankedType , b) ; self . relate (a , b) }) ? ; } ty :: Invariant => { infcx . enter_forall (b , | b | { let a = infcx . instantiate_binder_with_fresh_vars (span , HigherRankedType , a) ; self . relate (a , b) }) ? ; infcx . enter_forall (a , | a | { let b = infcx . instantiate_binder_with_fresh_vars (span , HigherRankedType , b) ; self . relate (a , b) }) ? ; } ty :: Bivariant => { unreachable ! ("Expected bivariance to be handled in relate_with_variance") } } } Ok (a) } }}}
mkitem!{mkimpl!{impl < 'tcx > PredicateEmittingRelation < InferCtxt < 'tcx > > for TypeRelating < '_ , 'tcx > { fn span (& self) -> Span { self . trace . span () } fn param_env (& self) -> ty :: ParamEnv < 'tcx > { self . param_env } fn structurally_relate_aliases (& self) -> StructurallyRelateAliases { StructurallyRelateAliases :: No } fn register_predicates (& mut self , preds : impl IntoIterator < Item : ty :: Upcast < TyCtxt < 'tcx > , ty :: Predicate < 'tcx > > > ,) { self . obligations . extend (preds . into_iter () . map (| pred | { Obligation :: new (self . infcx . tcx , self . trace . cause . clone () , self . param_env , pred) })) } fn register_goals (& mut self , goals : impl IntoIterator < Item = Goal < 'tcx , ty :: Predicate < 'tcx > > >) { self . obligations . extend (goals . into_iter () . map (| goal | { Obligation :: new (self . infcx . tcx , self . trace . cause . clone () , goal . param_env , goal . predicate ,) })) } fn register_alias_relate_predicate (& mut self , a : Ty < 'tcx > , b : Ty < 'tcx >) { self . register_predicates ([ty :: Binder :: dummy (match self . ambient_variance { ty :: Covariant => ty :: PredicateKind :: AliasRelate (a . into () , b . into () , ty :: AliasRelationDirection :: Subtype ,) , ty :: Contravariant => ty :: PredicateKind :: AliasRelate (b . into () , a . into () , ty :: AliasRelationDirection :: Subtype ,) , ty :: Invariant => ty :: PredicateKind :: AliasRelate (a . into () , b . into () , ty :: AliasRelationDirection :: Equate ,) , ty :: Bivariant => { unreachable ! ("Expected bivariance to be handled in relate_with_variance") } })]) ; } }}}