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
mkuse!{use std :: assert_matches :: assert_matches ;}
mkuse!{use std :: ops :: ControlFlow ;}
mkuse!{use rustc_data_structures :: fx :: { FxIndexMap , FxIndexSet } ;}
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_errors :: struct_span_code_err ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: PolyTraitRef ;}
mkuse!{use rustc_hir :: def :: { DefKind , Res } ;}
mkuse!{use rustc_hir :: def_id :: { CRATE_DEF_ID , DefId , LocalDefId } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: { self as ty , IsSuggestable , Ty , TyCtxt , TypeSuperVisitable , TypeVisitable , TypeVisitableExt , TypeVisitor , Upcast , } ;}
mkuse!{use rustc_span :: { ErrorGuaranteed , Ident , Span , kw , sym } ;}
mkuse!{use rustc_trait_selection :: traits ;}
mkuse!{use smallvec :: SmallVec ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use super :: errors :: GenericsArgsErrExtend ;}
mkuse!{use crate :: errors ;}
mkuse!{use crate :: hir_ty_lowering :: { AssocItemQSelf , FeedConstTy , HirTyLowerer , PredicateFilter , RegionInferReason , } ;}
mkitem!{mkstruct!{# [derive (Debug , Default)] struct CollectedBound { # [doc = " `Trait`"] positive : bool , # [doc = " `?Trait`"] maybe : bool , # [doc = " `!Trait`"] negative : bool , }}}
mkitem!{mkimpl!{impl CollectedBound { # [doc = " Returns `true` if any of `Trait`, `?Trait` or `!Trait` were encountered."] fn any (& self) -> bool { self . positive || self . maybe || self . negative } }}}
mkitem!{mkstruct!{# [derive (Debug)] struct CollectedSizednessBounds { sized : CollectedBound , meta_sized : CollectedBound , pointee_sized : CollectedBound , }}}
mkitem!{mkimpl!{impl CollectedSizednessBounds { # [doc = " Returns `true` if any of `Trait`, `?Trait` or `!Trait` were encountered for `Sized`,"] # [doc = " `MetaSized` or `PointeeSized`."] fn any (& self) -> bool { self . sized . any () || self . meta_sized . any () || self . pointee_sized . any () } }}}

macro_rules! search_bounds_for_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function search_bounds_for in module {}", module_path!());
    };
}

mkfn!{
    search_bounds_for_introspect!();
    fn search_bounds_for < 'tcx > (hir_bounds : & 'tcx [hir :: GenericBound < 'tcx >] , self_ty_where_predicates : Option < (LocalDefId , & 'tcx [hir :: WherePredicate < 'tcx >]) > , mut f : impl FnMut (& 'tcx PolyTraitRef < 'tcx >) ,) { let mut search_bounds = | hir_bounds : & 'tcx [hir :: GenericBound < 'tcx >] | { for hir_bound in hir_bounds { let hir :: GenericBound :: Trait (ptr) = hir_bound else { continue ; } ; f (ptr) } } ; search_bounds (hir_bounds) ; if let Some ((self_ty , where_clause)) = self_ty_where_predicates { for clause in where_clause { if let hir :: WherePredicateKind :: BoundPredicate (pred) = clause . kind && pred . is_param_bound (self_ty . to_def_id ()) { search_bounds (pred . bounds) ; } } } }
}

macro_rules! collect_relaxed_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_relaxed_bounds in module {}", module_path!());
    };
}

mkfn!{
    collect_relaxed_bounds_introspect!();
    fn collect_relaxed_bounds < 'tcx > (hir_bounds : & 'tcx [hir :: GenericBound < 'tcx >] , self_ty_where_predicates : Option < (LocalDefId , & 'tcx [hir :: WherePredicate < 'tcx >]) > ,) -> SmallVec < [& 'tcx PolyTraitRef < 'tcx > ; 1] > { let mut relaxed_bounds : SmallVec < [_ ; 1] > = SmallVec :: new () ; search_bounds_for (hir_bounds , self_ty_where_predicates , | ptr | { if matches ! (ptr . modifiers . polarity , hir :: BoundPolarity :: Maybe (_)) { relaxed_bounds . push (ptr) ; } }) ; relaxed_bounds }
}

macro_rules! collect_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_bounds in module {}", module_path!());
    };
}

mkfn!{
    collect_bounds_introspect!();
    fn collect_bounds < 'a , 'tcx > (hir_bounds : & 'a [hir :: GenericBound < 'tcx >] , self_ty_where_predicates : Option < (LocalDefId , & 'tcx [hir :: WherePredicate < 'tcx >]) > , target_did : DefId ,) -> CollectedBound { let mut collect_into = CollectedBound :: default () ; search_bounds_for (hir_bounds , self_ty_where_predicates , | ptr | { if ! matches ! (ptr . trait_ref . path . res , Res :: Def (DefKind :: Trait , did) if did == target_did) { return ; } match ptr . modifiers . polarity { hir :: BoundPolarity :: Maybe (_) => collect_into . maybe = true , hir :: BoundPolarity :: Negative (_) => collect_into . negative = true , hir :: BoundPolarity :: Positive => collect_into . positive = true , } }) ; collect_into }
}

macro_rules! collect_sizedness_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_sizedness_bounds in module {}", module_path!());
    };
}

mkfn!{
    collect_sizedness_bounds_introspect!();
    fn collect_sizedness_bounds < 'tcx > (tcx : TyCtxt < 'tcx > , hir_bounds : & 'tcx [hir :: GenericBound < 'tcx >] , self_ty_where_predicates : Option < (LocalDefId , & 'tcx [hir :: WherePredicate < 'tcx >]) > , span : Span ,) -> CollectedSizednessBounds { let sized_did = tcx . require_lang_item (hir :: LangItem :: Sized , span) ; let sized = collect_bounds (hir_bounds , self_ty_where_predicates , sized_did) ; let meta_sized_did = tcx . require_lang_item (hir :: LangItem :: MetaSized , span) ; let meta_sized = collect_bounds (hir_bounds , self_ty_where_predicates , meta_sized_did) ; let pointee_sized_did = tcx . require_lang_item (hir :: LangItem :: PointeeSized , span) ; let pointee_sized = collect_bounds (hir_bounds , self_ty_where_predicates , pointee_sized_did) ; CollectedSizednessBounds { sized , meta_sized , pointee_sized } }
}

macro_rules! add_trait_bound_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_trait_bound in module {}", module_path!());
    };
}

mkfn!{
    add_trait_bound_introspect!();
    # [doc = " Add a trait bound for `did`."] fn add_trait_bound < 'tcx > (tcx : TyCtxt < 'tcx > , bounds : & mut Vec < (ty :: Clause < 'tcx > , Span) > , self_ty : Ty < 'tcx > , did : DefId , span : Span ,) { let trait_ref = ty :: TraitRef :: new (tcx , did , [self_ty]) ; bounds . insert (0 , (trait_ref . upcast (tcx) , span)) ; }
}
mkitem!{mkimpl!{impl < 'tcx > dyn HirTyLowerer < 'tcx > + '_ { # [doc = " Adds sizedness bounds to a trait, trait alias, parameter, opaque type or associated type."] # [doc = ""] # [doc = " - On parameters, opaque type and associated types, add default `Sized` bound if no explicit"] # [doc = "   sizedness bounds are present."] # [doc = " - On traits and trait aliases, add default `MetaSized` supertrait if no explicit sizedness"] # [doc = "   bounds are present."] # [doc = " - On parameters, opaque type, associated types and trait aliases, add a `MetaSized` bound if"] # [doc = "   a `?Sized` bound is present."] pub (crate) fn add_sizedness_bounds (& self , bounds : & mut Vec < (ty :: Clause < 'tcx > , Span) > , self_ty : Ty < 'tcx > , hir_bounds : & 'tcx [hir :: GenericBound < 'tcx >] , self_ty_where_predicates : Option < (LocalDefId , & 'tcx [hir :: WherePredicate < 'tcx >]) > , trait_did : Option < LocalDefId > , span : Span ,) { let tcx = self . tcx () ; if tcx . has_attr (CRATE_DEF_ID , sym :: rustc_no_implicit_bounds) { return ; } let meta_sized_did = tcx . require_lang_item (hir :: LangItem :: MetaSized , span) ; let pointee_sized_did = tcx . require_lang_item (hir :: LangItem :: PointeeSized , span) ; if let Some (trait_did) = trait_did { let trait_did = trait_did . to_def_id () ; if trait_did == pointee_sized_did { return ; } if tcx . trait_is_auto (trait_did) { return ; } } else { let bounds = collect_relaxed_bounds (hir_bounds , self_ty_where_predicates) ; self . check_and_report_invalid_relaxed_bounds (bounds) ; } let collected = collect_sizedness_bounds (tcx , hir_bounds , self_ty_where_predicates , span) ; if (collected . sized . maybe || collected . sized . negative) && ! collected . sized . positive && ! collected . meta_sized . any () && ! collected . pointee_sized . any () { add_trait_bound (tcx , bounds , self_ty , meta_sized_did , span) ; } else if ! collected . any () { if trait_did . is_some () { add_trait_bound (tcx , bounds , self_ty , meta_sized_did , span) ; } else { let sized_did = tcx . require_lang_item (hir :: LangItem :: Sized , span) ; add_trait_bound (tcx , bounds , self_ty , sized_did , span) ; } } } # [doc = " Adds `experimental_default_bounds` bounds to the supertrait bounds."] pub (crate) fn add_default_super_traits (& self , trait_def_id : LocalDefId , bounds : & mut Vec < (ty :: Clause < 'tcx > , Span) > , hir_bounds : & 'tcx [hir :: GenericBound < 'tcx >] , hir_generics : & 'tcx hir :: Generics < 'tcx > , span : Span ,) { assert_matches ! (self . tcx () . def_kind (trait_def_id) , DefKind :: Trait | DefKind :: TraitAlias) ; if self . tcx () . trait_is_auto (trait_def_id . to_def_id ()) { return ; } self . add_default_traits (bounds , self . tcx () . types . self_param , hir_bounds , Some ((trait_def_id , hir_generics . predicates)) , span ,) ; } pub (crate) fn add_default_traits (& self , bounds : & mut Vec < (ty :: Clause < 'tcx > , Span) > , self_ty : Ty < 'tcx > , hir_bounds : & [hir :: GenericBound < 'tcx >] , self_ty_where_predicates : Option < (LocalDefId , & 'tcx [hir :: WherePredicate < 'tcx >]) > , span : Span ,) { self . tcx () . default_traits () . iter () . for_each (| default_trait | { self . add_default_trait (* default_trait , bounds , self_ty , hir_bounds , self_ty_where_predicates , span ,) ; }) ; } # [doc = " Add a `experimental_default_bounds` bound to the `bounds` if appropriate."] # [doc = ""] # [doc = " Doesn't add the bound if the HIR bounds contain any of `Trait`, `?Trait` or `!Trait`."] pub (crate) fn add_default_trait (& self , trait_ : hir :: LangItem , bounds : & mut Vec < (ty :: Clause < 'tcx > , Span) > , self_ty : Ty < 'tcx > , hir_bounds : & [hir :: GenericBound < 'tcx >] , self_ty_where_predicates : Option < (LocalDefId , & 'tcx [hir :: WherePredicate < 'tcx >]) > , span : Span ,) { let tcx = self . tcx () ; let trait_id = tcx . lang_items () . get (trait_) ; if let Some (trait_id) = trait_id && self . should_add_default_traits (trait_id , hir_bounds , self_ty_where_predicates) { add_trait_bound (tcx , bounds , self_ty , trait_id , span) ; } } # [doc = " Returns `true` if default trait bound should be added."] fn should_add_default_traits < 'a > (& self , trait_def_id : DefId , hir_bounds : & 'a [hir :: GenericBound < 'tcx >] , self_ty_where_predicates : Option < (LocalDefId , & 'tcx [hir :: WherePredicate < 'tcx >]) > ,) -> bool { let collected = collect_bounds (hir_bounds , self_ty_where_predicates , trait_def_id) ; ! self . tcx () . has_attr (CRATE_DEF_ID , sym :: rustc_no_implicit_bounds) && ! collected . any () } # [doc = " Lower HIR bounds into `bounds` given the self type `param_ty` and the overarching late-bound vars if any."] # [doc = ""] # [doc = " ### Examples"] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = " fn foo<T>() where for<'a> T: Trait<'a> + Copy {}"] # [doc = " //                ^^^^^^^ ^  ^^^^^^^^^^^^^^^^ `hir_bounds`, in HIR form"] # [doc = " //                |       |"] # [doc = " //                |       `param_ty`, in ty form"] # [doc = " //                `bound_vars`, in ty form"] # [doc = ""] # [doc = " fn bar<T>() where T: for<'a> Trait<'a> + Copy {} // no overarching `bound_vars` here!"] # [doc = " //                ^  ^^^^^^^^^^^^^^^^^^^^^^^^ `hir_bounds`, in HIR form"] # [doc = " //                |"] # [doc = " //                `param_ty`, in ty form"] # [doc = " ```"] # [doc = ""] # [doc = " ### A Note on Binders"] # [doc = ""] # [doc = " There is an implied binder around `param_ty` and `hir_bounds`."] # [doc = " See `lower_poly_trait_ref` for more details."] # [instrument (level = "debug" , skip (self , hir_bounds , bounds))] pub (crate) fn lower_bounds < 'hir , I : IntoIterator < Item = & 'hir hir :: GenericBound < 'tcx > > > (& self , param_ty : Ty < 'tcx > , hir_bounds : I , bounds : & mut Vec < (ty :: Clause < 'tcx > , Span) > , bound_vars : & 'tcx ty :: List < ty :: BoundVariableKind > , predicate_filter : PredicateFilter ,) where 'tcx : 'hir , { for hir_bound in hir_bounds { if let PredicateFilter :: SelfTraitThatDefines (assoc_ident) = predicate_filter { if let Some (trait_ref) = hir_bound . trait_ref () && let Some (trait_did) = trait_ref . trait_def_id () && self . tcx () . trait_may_define_assoc_item (trait_did , assoc_ident) { } else { continue ; } } match hir_bound { hir :: GenericBound :: Trait (poly_trait_ref) => { let _ = self . lower_poly_trait_ref (poly_trait_ref , param_ty , bounds , predicate_filter ,) ; } hir :: GenericBound :: Outlives (lifetime) => { if matches ! (predicate_filter , PredicateFilter :: ConstIfConst | PredicateFilter :: SelfConstIfConst) { continue ; } let region = self . lower_lifetime (lifetime , RegionInferReason :: OutlivesBound) ; let bound = ty :: Binder :: bind_with_vars (ty :: ClauseKind :: TypeOutlives (ty :: OutlivesPredicate (param_ty , region)) , bound_vars ,) ; bounds . push ((bound . upcast (self . tcx ()) , lifetime . ident . span)) ; } hir :: GenericBound :: Use (..) => { } } } } # [doc = " Lower an associated item constraint from the HIR into `bounds`."] # [doc = ""] # [doc = " ### A Note on Binders"] # [doc = ""] # [doc = " Given something like `T: for<'a> Iterator<Item = &'a u32>`,"] # [doc = " the `trait_ref` here will be `for<'a> T: Iterator`."] # [doc = " The `constraint` data however is from *inside* the binder"] # [doc = " (e.g., `&'a u32`) and hence may reference bound regions."] # [instrument (level = "debug" , skip (self , bounds , duplicates , path_span))] pub (super) fn lower_assoc_item_constraint (& self , hir_ref_id : hir :: HirId , trait_ref : ty :: PolyTraitRef < 'tcx > , constraint : & hir :: AssocItemConstraint < 'tcx > , bounds : & mut Vec < (ty :: Clause < 'tcx > , Span) > , duplicates : & mut FxIndexMap < DefId , Span > , path_span : Span , predicate_filter : PredicateFilter ,) -> Result < () , ErrorGuaranteed > { let tcx = self . tcx () ; let assoc_tag = if constraint . gen_args . parenthesized == hir :: GenericArgsParentheses :: ReturnTypeNotation { ty :: AssocTag :: Fn } else if let hir :: AssocItemConstraintKind :: Equality { term : hir :: Term :: Const (_) } = constraint . kind { ty :: AssocTag :: Const } else { ty :: AssocTag :: Type } ; let candidate = if self . probe_trait_that_defines_assoc_item (trait_ref . def_id () , assoc_tag , constraint . ident ,) { trait_ref } else { self . probe_single_bound_for_assoc_item (| | traits :: supertraits (tcx , trait_ref) , AssocItemQSelf :: Trait (trait_ref . def_id ()) , assoc_tag , constraint . ident , path_span , Some (constraint) ,) ? } ; let assoc_item = self . probe_assoc_item (constraint . ident , assoc_tag , hir_ref_id , constraint . span , candidate . def_id () ,) . expect ("failed to find associated item") ; duplicates . entry (assoc_item . def_id) . and_modify (| prev_span | { self . dcx () . emit_err (errors :: ValueOfAssociatedStructAlreadySpecified { span : constraint . span , prev_span : * prev_span , item_name : constraint . ident , def_path : tcx . def_path_str (assoc_item . container_id (tcx)) , }) ; }) . or_insert (constraint . span) ; let projection_term = if let ty :: AssocTag :: Fn = assoc_tag { let bound_vars = tcx . late_bound_vars (constraint . hir_id) ; ty :: Binder :: bind_with_vars (self . lower_return_type_notation_ty (candidate , assoc_item . def_id , path_span) ? . into () , bound_vars ,) } else { let alias_term = candidate . map_bound (| trait_ref | { let item_segment = hir :: PathSegment { ident : constraint . ident , hir_id : constraint . hir_id , res : Res :: Err , args : Some (constraint . gen_args) , infer_args : false , } ; let alias_args = self . lower_generic_args_of_assoc_item (path_span , assoc_item . def_id , & item_segment , trait_ref . args ,) ; debug ! (? alias_args) ; ty :: AliasTerm :: new_from_args (tcx , assoc_item . def_id , alias_args) }) ; if let Some (const_arg) = constraint . ct () && let hir :: ConstArgKind :: Anon (anon_const) = const_arg . kind { let ty = alias_term . map_bound (| alias | tcx . type_of (alias . def_id) . instantiate (tcx , alias . args)) ; let ty = check_assoc_const_binding_type (self , constraint . ident , ty , constraint . hir_id) ; tcx . feed_anon_const_type (anon_const . def_id , ty :: EarlyBinder :: bind (ty)) ; } alias_term } ; match constraint . kind { hir :: AssocItemConstraintKind :: Equality { .. } if let ty :: AssocTag :: Fn = assoc_tag => { return Err (self . dcx () . emit_err (crate :: errors :: ReturnTypeNotationEqualityBound { span : constraint . span , })) ; } hir :: AssocItemConstraintKind :: Equality { term } => { let term = match term { hir :: Term :: Ty (ty) => self . lower_ty (ty) . into () , hir :: Term :: Const (ct) => self . lower_const_arg (ct , FeedConstTy :: No) . into () , } ; let late_bound_in_projection_ty = tcx . collect_constrained_late_bound_regions (projection_term) ; let late_bound_in_term = tcx . collect_referenced_late_bound_regions (trait_ref . rebind (term)) ; debug ! (? late_bound_in_projection_ty) ; debug ! (? late_bound_in_term) ; self . validate_late_bound_regions (late_bound_in_projection_ty , late_bound_in_term , | br_name | { struct_span_code_err ! (self . dcx () , constraint . span , E0582 , "binding for associated type `{}` references {}, \
                             which does not appear in the trait input types" , constraint . ident , br_name) } ,) ; match predicate_filter { PredicateFilter :: All | PredicateFilter :: SelfOnly | PredicateFilter :: SelfAndAssociatedTypeBounds => { let bound = projection_term . map_bound (| projection_term | { ty :: ClauseKind :: Projection (ty :: ProjectionPredicate { projection_term , term , }) }) ; bounds . push ((bound . upcast (tcx) , constraint . span)) ; } PredicateFilter :: SelfTraitThatDefines (_) => { } PredicateFilter :: ConstIfConst | PredicateFilter :: SelfConstIfConst => { } } } hir :: AssocItemConstraintKind :: Bound { bounds : hir_bounds } => { match predicate_filter { PredicateFilter :: All | PredicateFilter :: SelfAndAssociatedTypeBounds | PredicateFilter :: ConstIfConst => { let projection_ty = projection_term . map_bound (| projection_term | projection_term . expect_ty (self . tcx ())) ; let param_ty = Ty :: new_alias (tcx , ty :: Projection , projection_ty . skip_binder ()) ; self . lower_bounds (param_ty , hir_bounds , bounds , projection_ty . bound_vars () , predicate_filter ,) ; } PredicateFilter :: SelfOnly | PredicateFilter :: SelfTraitThatDefines (_) | PredicateFilter :: SelfConstIfConst => { } } } } Ok (()) } # [doc = " Lower a type, possibly specially handling the type if it's a return type notation"] # [doc = " which we otherwise deny in other positions."] pub fn lower_ty_maybe_return_type_notation (& self , hir_ty : & hir :: Ty < 'tcx >) -> Ty < 'tcx > { let hir :: TyKind :: Path (qpath) = hir_ty . kind else { return self . lower_ty (hir_ty) ; } ; let tcx = self . tcx () ; match qpath { hir :: QPath :: Resolved (opt_self_ty , path) if let [mod_segments @ .. , trait_segment , item_segment] = & path . segments [..] && item_segment . args . is_some_and (| args | { matches ! (args . parenthesized , hir :: GenericArgsParentheses :: ReturnTypeNotation) }) => { let _ = self . prohibit_generic_args (mod_segments . iter () , GenericsArgsErrExtend :: None) ; let item_def_id = match path . res { Res :: Def (DefKind :: AssocFn , item_def_id) => item_def_id , Res :: Err => { return Ty :: new_error_with_message (tcx , hir_ty . span , "failed to resolve RTN" ,) ; } _ => bug ! ("only expected method resolution for fully qualified RTN") , } ; let trait_def_id = tcx . parent (item_def_id) ; let Some (self_ty) = opt_self_ty else { let guar = self . report_missing_self_ty_for_resolved_path (trait_def_id , hir_ty . span , item_segment , ty :: AssocTag :: Type ,) ; return Ty :: new_error (tcx , guar) ; } ; let self_ty = self . lower_ty (self_ty) ; let trait_ref = self . lower_mono_trait_ref (hir_ty . span , trait_def_id , self_ty , trait_segment , false ,) ; let candidate = ty :: Binder :: bind_with_vars (trait_ref , tcx . late_bound_vars (item_segment . hir_id)) ; match self . lower_return_type_notation_ty (candidate , item_def_id , hir_ty . span) { Ok (ty) => Ty :: new_alias (tcx , ty :: Projection , ty) , Err (guar) => Ty :: new_error (tcx , guar) , } } hir :: QPath :: TypeRelative (hir_self_ty , segment) if segment . args . is_some_and (| args | { matches ! (args . parenthesized , hir :: GenericArgsParentheses :: ReturnTypeNotation) }) => { let self_ty = self . lower_ty (hir_self_ty) ; let (item_def_id , bound) = match self . resolve_type_relative_path (self_ty , hir_self_ty , ty :: AssocTag :: Fn , segment , hir_ty . hir_id , hir_ty . span , None ,) { Ok (result) => result , Err (guar) => return Ty :: new_error (tcx , guar) , } ; if bound . has_bound_vars () { return Ty :: new_error (tcx , self . dcx () . emit_err (errors :: AssociatedItemTraitUninferredGenericParams { span : hir_ty . span , inferred_sugg : Some (hir_ty . span . with_hi (segment . ident . span . lo ())) , bound : format ! ("{}::" , tcx . anonymize_bound_vars (bound) . skip_binder ()) , mpart_sugg : None , what : tcx . def_descr (item_def_id) , }) ,) ; } match self . lower_return_type_notation_ty (bound , item_def_id , hir_ty . span) { Ok (ty) => Ty :: new_alias (tcx , ty :: Projection , ty) , Err (guar) => Ty :: new_error (tcx , guar) , } } _ => self . lower_ty (hir_ty) , } } # [doc = " Do the common parts of lowering an RTN type. This involves extending the"] # [doc = " candidate binder to include all of the early- and late-bound vars that are"] # [doc = " defined on the function itself, and constructing a projection to the RPITIT"] # [doc = " return type of that function."] fn lower_return_type_notation_ty (& self , candidate : ty :: PolyTraitRef < 'tcx > , item_def_id : DefId , path_span : Span ,) -> Result < ty :: AliasTy < 'tcx > , ErrorGuaranteed > { let tcx = self . tcx () ; let mut emitted_bad_param_err = None ; let mut num_bound_vars = candidate . bound_vars () . len () ; let args = candidate . skip_binder () . args . extend_to (tcx , item_def_id , | param , _ | { let arg = match param . kind { ty :: GenericParamDefKind :: Lifetime => ty :: Region :: new_bound (tcx , ty :: INNERMOST , ty :: BoundRegion { var : ty :: BoundVar :: from_usize (num_bound_vars) , kind : ty :: BoundRegionKind :: Named (param . def_id) , } ,) . into () , ty :: GenericParamDefKind :: Type { .. } => { let guar = * emitted_bad_param_err . get_or_insert_with (| | { self . dcx () . emit_err (crate :: errors :: ReturnTypeNotationIllegalParam :: Type { span : path_span , param_span : tcx . def_span (param . def_id) , }) }) ; Ty :: new_error (tcx , guar) . into () } ty :: GenericParamDefKind :: Const { .. } => { let guar = * emitted_bad_param_err . get_or_insert_with (| | { self . dcx () . emit_err (crate :: errors :: ReturnTypeNotationIllegalParam :: Const { span : path_span , param_span : tcx . def_span (param . def_id) , }) }) ; ty :: Const :: new_error (tcx , guar) . into () } } ; num_bound_vars += 1 ; arg }) ; let output = tcx . fn_sig (item_def_id) . skip_binder () . output () ; let output = if let ty :: Alias (ty :: Projection , alias_ty) = * output . skip_binder () . kind () && tcx . is_impl_trait_in_trait (alias_ty . def_id) { alias_ty } else { return Err (self . dcx () . emit_err (crate :: errors :: ReturnTypeNotationOnNonRpitit { span : path_span , ty : tcx . liberate_late_bound_regions (item_def_id , output) , fn_span : tcx . hir_span_if_local (item_def_id) , note : () , })) ; } ; let shifted_output = tcx . shift_bound_var_indices (num_bound_vars , output) ; Ok (ty :: EarlyBinder :: bind (shifted_output) . instantiate (tcx , args)) } }}}

macro_rules! check_assoc_const_binding_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_assoc_const_binding_type in module {}", module_path!());
    };
}

mkfn!{
    check_assoc_const_binding_type_introspect!();
    # [doc = " Detect and reject early-bound & escaping late-bound generic params in the type of assoc const bindings."] # [doc = ""] # [doc = " FIXME(const_generics): This is a temporary and semi-artificial restriction until the"] # [doc = " arrival of *generic const generics*[^1]."] # [doc = ""] # [doc = " It might actually be possible that we can already support early-bound generic params"] # [doc = " in such types if we just lifted some more checks in other places, too, for example"] # [doc = " inside `HirTyLowerer::lower_anon_const`. However, even if that were the case, we should"] # [doc = " probably gate this behind another feature flag."] # [doc = ""] # [doc = " [^1]: <https://github.com/rust-lang/project-const-generics/issues/28>."] fn check_assoc_const_binding_type < 'tcx > (cx : & dyn HirTyLowerer < 'tcx > , assoc_const : Ident , ty : ty :: Binder < 'tcx , Ty < 'tcx > > , hir_id : hir :: HirId ,) -> Ty < 'tcx > { let ty = ty . skip_binder () ; if ! ty . has_param () && ! ty . has_escaping_bound_vars () { return ty ; } let mut collector = GenericParamAndBoundVarCollector { cx , params : Default :: default () , vars : Default :: default () , depth : ty :: INNERMOST , } ; let mut guar = ty . visit_with (& mut collector) . break_value () ; let tcx = cx . tcx () ; let ty_note = ty . make_suggestable (tcx , false , None) . map (| ty | crate :: errors :: TyOfAssocConstBindingNote { assoc_const , ty }) ; let enclosing_item_owner_id = tcx . hir_parent_owner_iter (hir_id) . find_map (| (owner_id , parent) | parent . generics () . map (| _ | owner_id)) . unwrap () ; let generics = tcx . generics_of (enclosing_item_owner_id) ; for index in collector . params { let param = generics . param_at (index as _ , tcx) ; let is_self_param = param . name == kw :: SelfUpper ; guar . get_or_insert (cx . dcx () . emit_err (crate :: errors :: ParamInTyOfAssocConstBinding { span : assoc_const . span , assoc_const , param_name : param . name , param_def_kind : tcx . def_descr (param . def_id) , param_category : if is_self_param { "self" } else if param . kind . is_synthetic () { "synthetic" } else { "normal" } , param_defined_here_label : (! is_self_param) . then (| | tcx . def_ident_span (param . def_id) . unwrap ()) , ty_note , })) ; } for var_def_id in collector . vars { guar . get_or_insert (cx . dcx () . emit_err (crate :: errors :: EscapingBoundVarInTyOfAssocConstBinding { span : assoc_const . span , assoc_const , var_name : cx . tcx () . item_name (var_def_id) , var_def_kind : tcx . def_descr (var_def_id) , var_defined_here_label : tcx . def_ident_span (var_def_id) . unwrap () , ty_note , } ,)) ; } let guar = guar . unwrap_or_else (| | bug ! ("failed to find gen params or bound vars in ty")) ; Ty :: new_error (tcx , guar) }
}
mkitem!{mkstruct!{struct GenericParamAndBoundVarCollector < 'a , 'tcx > { cx : & 'a dyn HirTyLowerer < 'tcx > , params : FxIndexSet < u32 > , vars : FxIndexSet < DefId > , depth : ty :: DebruijnIndex , }}}
mkitem!{mkimpl!{impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for GenericParamAndBoundVarCollector < '_ , 'tcx > { type Result = ControlFlow < ErrorGuaranteed > ; fn visit_binder < T : TypeVisitable < TyCtxt < 'tcx > > > (& mut self , binder : & ty :: Binder < 'tcx , T > ,) -> Self :: Result { self . depth . shift_in (1) ; let result = binder . super_visit_with (self) ; self . depth . shift_out (1) ; result } fn visit_ty (& mut self , ty : Ty < 'tcx >) -> Self :: Result { match ty . kind () { ty :: Param (param) => { self . params . insert (param . index) ; } ty :: Bound (db , bt) if * db >= self . depth => { self . vars . insert (match bt . kind { ty :: BoundTyKind :: Param (def_id) => def_id , ty :: BoundTyKind :: Anon => { let reported = self . cx . dcx () . delayed_bug (format ! ("unexpected anon bound ty: {:?}" , bt . var)) ; return ControlFlow :: Break (reported) ; } }) ; } _ if ty . has_param () || ty . has_bound_vars () => return ty . super_visit_with (self) , _ => { } } ControlFlow :: Continue (()) } fn visit_region (& mut self , re : ty :: Region < 'tcx >) -> Self :: Result { match re . kind () { ty :: ReEarlyParam (param) => { self . params . insert (param . index) ; } ty :: ReBound (db , br) if db >= self . depth => { self . vars . insert (match br . kind { ty :: BoundRegionKind :: Named (def_id) => def_id , ty :: BoundRegionKind :: Anon | ty :: BoundRegionKind :: ClosureEnv => { let guar = self . cx . dcx () . delayed_bug (format ! ("unexpected bound region kind: {:?}" , br . kind)) ; return ControlFlow :: Break (guar) ; } ty :: BoundRegionKind :: NamedAnon (_) => bug ! ("only used for pretty printing") , }) ; } _ => { } } ControlFlow :: Continue (()) } fn visit_const (& mut self , ct : ty :: Const < 'tcx >) -> Self :: Result { match ct . kind () { ty :: ConstKind :: Param (param) => { self . params . insert (param . index) ; } ty :: ConstKind :: Bound (db , _) if db >= self . depth => { let guar = self . cx . dcx () . delayed_bug ("unexpected escaping late-bound const var") ; return ControlFlow :: Break (guar) ; } _ if ct . has_param () || ct . has_bound_vars () => return ct . super_visit_with (self) , _ => { } } ControlFlow :: Continue (()) } }}}