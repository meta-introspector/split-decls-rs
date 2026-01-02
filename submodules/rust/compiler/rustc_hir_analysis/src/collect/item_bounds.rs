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
mkuse!{use rustc_data_structures :: fx :: { FxIndexMap , FxIndexSet } ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_infer :: traits :: util ;}
mkuse!{use rustc_middle :: ty :: { self , GenericArgs , Ty , TyCtxt , TypeFoldable , TypeFolder , TypeSuperFoldable , TypeVisitableExt , Upcast , shift_vars , } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use rustc_span :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use super :: ItemCtxt ;}
mkuse!{use super :: predicates_of :: assert_only_contains_predicates_from ;}
mkuse!{use crate :: hir_ty_lowering :: { HirTyLowerer , PredicateFilter } ;}

macro_rules! associated_type_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function associated_type_bounds in module {}", module_path!());
    };
}

mkfn!{
    associated_type_bounds_introspect!();
    # [doc = " For associated types we include both bounds written on the type"] # [doc = " (`type X: Trait`) and predicates from the trait: `where Self::X: Trait`."] # [doc = ""] # [doc = " Note that this filtering is done with the items identity args to"] # [doc = " simplify checking that these bounds are met in impls. This means that"] # [doc = " a bound such as `for<'b> <Self as X<'b>>::U: Clone` can't be used, as in"] # [doc = " `hr-associated-type-bound-1.rs`."] fn associated_type_bounds < 'tcx > (tcx : TyCtxt < 'tcx > , assoc_item_def_id : LocalDefId , hir_bounds : & 'tcx [hir :: GenericBound < 'tcx >] , span : Span , filter : PredicateFilter ,) -> & 'tcx [(ty :: Clause < 'tcx > , Span)] { ty :: print :: with_reduced_queries ! ({ let item_ty = Ty :: new_projection_from_args (tcx , assoc_item_def_id . to_def_id () , GenericArgs :: identity_for_item (tcx , assoc_item_def_id) ,) ; let icx = ItemCtxt :: new (tcx , assoc_item_def_id) ; let mut bounds = Vec :: new () ; icx . lowerer () . lower_bounds (item_ty , hir_bounds , & mut bounds , ty :: List :: empty () , filter) ; match filter { PredicateFilter :: All | PredicateFilter :: SelfOnly | PredicateFilter :: SelfTraitThatDefines (_) | PredicateFilter :: SelfAndAssociatedTypeBounds => { icx . lowerer () . add_sizedness_bounds (& mut bounds , item_ty , hir_bounds , None , None , span ,) ; icx . lowerer () . add_default_traits (& mut bounds , item_ty , hir_bounds , None , span) ; let trait_def_id = tcx . local_parent (assoc_item_def_id) ; let trait_predicates = tcx . trait_explicit_predicates_and_bounds (trait_def_id) ; let item_trait_ref = ty :: TraitRef :: identity (tcx , tcx . parent (assoc_item_def_id . to_def_id ())) ; bounds . extend (trait_predicates . predicates . iter () . copied () . filter_map (| (clause , span) | { remap_gat_vars_and_recurse_into_nested_projections (tcx , filter , item_trait_ref , assoc_item_def_id , span , clause ,) } ,)) ; } PredicateFilter :: ConstIfConst | PredicateFilter :: SelfConstIfConst => { } } let bounds = tcx . arena . alloc_from_iter (bounds) ; debug ! ("associated_type_bounds({}) = {:?}" , tcx . def_path_str (assoc_item_def_id . to_def_id ()) , bounds) ; assert_only_contains_predicates_from (filter , bounds , item_ty) ; bounds }) }
}

macro_rules! remap_gat_vars_and_recurse_into_nested_projections_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remap_gat_vars_and_recurse_into_nested_projections in module {}", module_path!());
    };
}

mkfn!{
    remap_gat_vars_and_recurse_into_nested_projections_introspect!();
    # [doc = " The code below is quite involved, so let me explain."] # [doc = ""] # [doc = " We loop here, because we also want to collect vars for nested associated items as"] # [doc = " well. For example, given a clause like `Self::A::B`, we want to add that to the"] # [doc = " item bounds for `A`, so that we may use that bound in the case that `Self::A::B` is"] # [doc = " rigid."] # [doc = ""] # [doc = " Secondly, regarding bound vars, when we see a where clause that mentions a GAT"] # [doc = " like `for<'a, ...> Self::Assoc<'a, ...>: Bound<'b, ...>`, we want to turn that into"] # [doc = " an item bound on the GAT, where all of the GAT args are substituted with the GAT's"] # [doc = " param regions, and then keep all of the other late-bound vars in the bound around."] # [doc = " We need to \"compress\" the binder so that it doesn't mention any of those vars that"] # [doc = " were mapped to params."] fn remap_gat_vars_and_recurse_into_nested_projections < 'tcx > (tcx : TyCtxt < 'tcx > , filter : PredicateFilter , item_trait_ref : ty :: TraitRef < 'tcx > , assoc_item_def_id : LocalDefId , span : Span , clause : ty :: Clause < 'tcx > ,) -> Option < (ty :: Clause < 'tcx > , Span) > { let mut clause_ty = match clause . kind () . skip_binder () { ty :: ClauseKind :: Trait (tr) => tr . self_ty () , ty :: ClauseKind :: Projection (proj) => proj . projection_term . self_ty () , ty :: ClauseKind :: TypeOutlives (outlives) => outlives . 0 , ty :: ClauseKind :: HostEffect (host) => host . self_ty () , _ => return None , } ; let gat_vars = loop { if let ty :: Alias (ty :: Projection , alias_ty) = * clause_ty . kind () { if alias_ty . trait_ref (tcx) == item_trait_ref && alias_ty . def_id == assoc_item_def_id . to_def_id () { break & alias_ty . args [item_trait_ref . args . len () ..] ; } else { match filter { PredicateFilter :: All => { } PredicateFilter :: SelfOnly => { return None ; } PredicateFilter :: SelfTraitThatDefines (_) | PredicateFilter :: SelfConstIfConst | PredicateFilter :: SelfAndAssociatedTypeBounds | PredicateFilter :: ConstIfConst => { unreachable ! ("invalid predicate filter for \
                            `remap_gat_vars_and_recurse_into_nested_projections`") } } clause_ty = alias_ty . self_ty () ; continue ; } } return None ; } ; if gat_vars . is_empty () { return Some ((clause , span)) ; } let mut mapping = FxIndexMap :: default () ; let generics = tcx . generics_of (assoc_item_def_id) ; for (param , var) in std :: iter :: zip (& generics . own_params , gat_vars) { let existing = match var . kind () { ty :: GenericArgKind :: Lifetime (re) => { if let ty :: RegionKind :: ReBound (ty :: INNERMOST , bv) = re . kind () { mapping . insert (bv . var , tcx . mk_param_from_def (param)) } else { return None ; } } ty :: GenericArgKind :: Type (ty) => { if let ty :: Bound (ty :: INNERMOST , bv) = * ty . kind () { mapping . insert (bv . var , tcx . mk_param_from_def (param)) } else { return None ; } } ty :: GenericArgKind :: Const (ct) => { if let ty :: ConstKind :: Bound (ty :: INNERMOST , bv) = ct . kind () { mapping . insert (bv . var , tcx . mk_param_from_def (param)) } else { return None ; } } } ; if existing . is_some () { return None ; } } let mut folder = MapAndCompressBoundVars { tcx , binder : ty :: INNERMOST , still_bound_vars : vec ! [] , mapping } ; let pred = clause . kind () . skip_binder () . fold_with (& mut folder) ; Some ((ty :: Binder :: bind_with_vars (pred , tcx . mk_bound_variable_kinds (& folder . still_bound_vars)) . upcast (tcx) , span ,)) }
}
mkitem!{mkstruct!{# [doc = " Given some where clause like `for<'b, 'c> <Self as Trait<'a_identity>>::Gat<'b>: Bound<'c>`,"] # [doc = " the mapping will map `'b` back to the GAT's `'b_identity`. Then we need to compress the"] # [doc = " remaining bound var `'c` to index 0."] # [doc = ""] # [doc = " This folder gives us: `for<'c> <Self as Trait<'a_identity>>::Gat<'b_identity>: Bound<'c>`,"] # [doc = " which is sufficient for an item bound for `Gat`, since all of the GAT's args are identity."] struct MapAndCompressBoundVars < 'tcx > { tcx : TyCtxt < 'tcx > , # [doc = " How deep are we? Makes sure we don't touch the vars of nested binders."] binder : ty :: DebruijnIndex , # [doc = " List of bound vars that remain unsubstituted because they were not"] # [doc = " mentioned in the GAT's args."] still_bound_vars : Vec < ty :: BoundVariableKind > , # [doc = " Subtle invariant: If the `GenericArg` is bound, then it should be"] # [doc = " stored with the debruijn index of `INNERMOST` so it can be shifted"] # [doc = " correctly during substitution."] mapping : FxIndexMap < ty :: BoundVar , ty :: GenericArg < 'tcx > > , }}}
mkitem!{mkimpl!{impl < 'tcx > TypeFolder < TyCtxt < 'tcx > > for MapAndCompressBoundVars < 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn fold_binder < T > (& mut self , t : ty :: Binder < 'tcx , T >) -> ty :: Binder < 'tcx , T > where ty :: Binder < 'tcx , T > : TypeSuperFoldable < TyCtxt < 'tcx > > , { self . binder . shift_in (1) ; let out = t . super_fold_with (self) ; self . binder . shift_out (1) ; out } fn fold_ty (& mut self , ty : Ty < 'tcx >) -> Ty < 'tcx > { if ! ty . has_bound_vars () { return ty ; } if let ty :: Bound (binder , old_bound) = * ty . kind () && self . binder == binder { let mapped = if let Some (mapped) = self . mapping . get (& old_bound . var) { mapped . expect_ty () } else { let var = ty :: BoundVar :: from_usize (self . still_bound_vars . len ()) ; self . still_bound_vars . push (ty :: BoundVariableKind :: Ty (old_bound . kind)) ; let mapped = Ty :: new_bound (self . tcx , ty :: INNERMOST , ty :: BoundTy { var , kind : old_bound . kind } ,) ; self . mapping . insert (old_bound . var , mapped . into ()) ; mapped } ; shift_vars (self . tcx , mapped , self . binder . as_u32 ()) } else { ty . super_fold_with (self) } } fn fold_region (& mut self , re : ty :: Region < 'tcx >) -> ty :: Region < 'tcx > { if let ty :: ReBound (binder , old_bound) = re . kind () && self . binder == binder { let mapped = if let Some (mapped) = self . mapping . get (& old_bound . var) { mapped . expect_region () } else { let var = ty :: BoundVar :: from_usize (self . still_bound_vars . len ()) ; self . still_bound_vars . push (ty :: BoundVariableKind :: Region (old_bound . kind)) ; let mapped = ty :: Region :: new_bound (self . tcx , ty :: INNERMOST , ty :: BoundRegion { var , kind : old_bound . kind } ,) ; self . mapping . insert (old_bound . var , mapped . into ()) ; mapped } ; shift_vars (self . tcx , mapped , self . binder . as_u32 ()) } else { re } } fn fold_const (& mut self , ct : ty :: Const < 'tcx >) -> ty :: Const < 'tcx > { if ! ct . has_bound_vars () { return ct ; } if let ty :: ConstKind :: Bound (binder , old_bound) = ct . kind () && self . binder == binder { let mapped = if let Some (mapped) = self . mapping . get (& old_bound . var) { mapped . expect_const () } else { let var = ty :: BoundVar :: from_usize (self . still_bound_vars . len ()) ; self . still_bound_vars . push (ty :: BoundVariableKind :: Const) ; let mapped = ty :: Const :: new_bound (self . tcx , ty :: INNERMOST , ty :: BoundConst { var }) ; self . mapping . insert (old_bound . var , mapped . into ()) ; mapped } ; shift_vars (self . tcx , mapped , self . binder . as_u32 ()) } else { ct . super_fold_with (self) } } fn fold_predicate (& mut self , p : ty :: Predicate < 'tcx >) -> ty :: Predicate < 'tcx > { if ! p . has_bound_vars () { p } else { p . super_fold_with (self) } } }}}

macro_rules! opaque_type_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function opaque_type_bounds in module {}", module_path!());
    };
}

mkfn!{
    opaque_type_bounds_introspect!();
    # [doc = " Opaque types don't inherit bounds from their parent: for return position"] # [doc = " impl trait it isn't possible to write a suitable predicate on the"] # [doc = " containing function and for type-alias impl trait we don't have a backwards"] # [doc = " compatibility issue."] # [instrument (level = "trace" , skip (tcx , item_ty))] fn opaque_type_bounds < 'tcx > (tcx : TyCtxt < 'tcx > , opaque_def_id : LocalDefId , hir_bounds : & 'tcx [hir :: GenericBound < 'tcx >] , item_ty : Ty < 'tcx > , span : Span , filter : PredicateFilter ,) -> & 'tcx [(ty :: Clause < 'tcx > , Span)] { ty :: print :: with_reduced_queries ! ({ let icx = ItemCtxt :: new (tcx , opaque_def_id) ; let mut bounds = Vec :: new () ; icx . lowerer () . lower_bounds (item_ty , hir_bounds , & mut bounds , ty :: List :: empty () , filter) ; match filter { PredicateFilter :: All | PredicateFilter :: SelfOnly | PredicateFilter :: SelfTraitThatDefines (_) | PredicateFilter :: SelfAndAssociatedTypeBounds => { icx . lowerer () . add_sizedness_bounds (& mut bounds , item_ty , hir_bounds , None , None , span ,) ; icx . lowerer () . add_default_traits (& mut bounds , item_ty , hir_bounds , None , span) ; } PredicateFilter :: ConstIfConst | PredicateFilter :: SelfConstIfConst => { } } debug ! (? bounds) ; tcx . arena . alloc_slice (& bounds) }) }
}

macro_rules! explicit_item_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function explicit_item_bounds in module {}", module_path!());
    };
}

mkfn!{
    explicit_item_bounds_introspect!();
    pub (super) fn explicit_item_bounds (tcx : TyCtxt < '_ > , def_id : LocalDefId ,) -> ty :: EarlyBinder < '_ , & '_ [(ty :: Clause < '_ > , Span)] > { explicit_item_bounds_with_filter (tcx , def_id , PredicateFilter :: All) }
}

macro_rules! explicit_item_self_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function explicit_item_self_bounds in module {}", module_path!());
    };
}

mkfn!{
    explicit_item_self_bounds_introspect!();
    pub (super) fn explicit_item_self_bounds (tcx : TyCtxt < '_ > , def_id : LocalDefId ,) -> ty :: EarlyBinder < '_ , & '_ [(ty :: Clause < '_ > , Span)] > { explicit_item_bounds_with_filter (tcx , def_id , PredicateFilter :: SelfOnly) }
}

macro_rules! explicit_item_bounds_with_filter_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function explicit_item_bounds_with_filter in module {}", module_path!());
    };
}

mkfn!{
    explicit_item_bounds_with_filter_introspect!();
    pub (super) fn explicit_item_bounds_with_filter (tcx : TyCtxt < '_ > , def_id : LocalDefId , filter : PredicateFilter ,) -> ty :: EarlyBinder < '_ , & '_ [(ty :: Clause < '_ > , Span)] > { match tcx . opt_rpitit_info (def_id . to_def_id ()) { Some (ty :: ImplTraitInTraitData :: Trait { opaque_def_id , .. }) => { let opaque_ty = tcx . hir_node_by_def_id (opaque_def_id . expect_local ()) . expect_opaque_ty () ; let bounds = associated_type_bounds (tcx , def_id , opaque_ty . bounds , opaque_ty . span , filter) ; return ty :: EarlyBinder :: bind (bounds) ; } Some (ty :: ImplTraitInTraitData :: Impl { .. }) => { span_bug ! (tcx . def_span (def_id) , "RPITIT in impl should not have item bounds") } None => { } } let bounds = match tcx . hir_node_by_def_id (def_id) { hir :: Node :: TraitItem (hir :: TraitItem { kind : hir :: TraitItemKind :: Type (bounds , _) , span , .. }) => associated_type_bounds (tcx , def_id , bounds , * span , filter) , hir :: Node :: OpaqueTy (hir :: OpaqueTy { bounds , origin , span , .. }) => match origin { rustc_hir :: OpaqueTyOrigin :: FnReturn { parent , in_trait_or_impl : Some (hir :: RpitContext :: Trait) , } | rustc_hir :: OpaqueTyOrigin :: AsyncFn { parent , in_trait_or_impl : Some (hir :: RpitContext :: Trait) , } => { let args = GenericArgs :: identity_for_item (tcx , def_id) ; let item_ty = Ty :: new_opaque (tcx , def_id . to_def_id () , args) ; let bounds = & * tcx . arena . alloc_slice (& opaque_type_bounds (tcx , def_id , bounds , item_ty , * span , filter) . to_vec () . fold_with (& mut AssocTyToOpaque { tcx , fn_def_id : parent . to_def_id () }) ,) ; assert_only_contains_predicates_from (filter , bounds , item_ty) ; bounds } rustc_hir :: OpaqueTyOrigin :: FnReturn { parent : _ , in_trait_or_impl : None | Some (hir :: RpitContext :: TraitImpl) , } | rustc_hir :: OpaqueTyOrigin :: AsyncFn { parent : _ , in_trait_or_impl : None | Some (hir :: RpitContext :: TraitImpl) , } | rustc_hir :: OpaqueTyOrigin :: TyAlias { parent : _ , .. } => { let args = GenericArgs :: identity_for_item (tcx , def_id) ; let item_ty = Ty :: new_opaque (tcx , def_id . to_def_id () , args) ; let bounds = opaque_type_bounds (tcx , def_id , bounds , item_ty , * span , filter) ; assert_only_contains_predicates_from (filter , bounds , item_ty) ; bounds } } , hir :: Node :: Item (hir :: Item { kind : hir :: ItemKind :: TyAlias (..) , .. }) => & [] , node => bug ! ("item_bounds called on {def_id:?} => {node:?}") , } ; ty :: EarlyBinder :: bind (bounds) }
}

macro_rules! item_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function item_bounds in module {}", module_path!());
    };
}

mkfn!{
    item_bounds_introspect!();
    pub (super) fn item_bounds (tcx : TyCtxt < '_ > , def_id : DefId) -> ty :: EarlyBinder < '_ , ty :: Clauses < '_ > > { tcx . explicit_item_bounds (def_id) . map_bound (| bounds | { tcx . mk_clauses_from_iter (util :: elaborate (tcx , bounds . iter () . map (| & (bound , _span) | bound))) }) }
}

macro_rules! item_self_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function item_self_bounds in module {}", module_path!());
    };
}

mkfn!{
    item_self_bounds_introspect!();
    pub (super) fn item_self_bounds (tcx : TyCtxt < '_ > , def_id : DefId ,) -> ty :: EarlyBinder < '_ , ty :: Clauses < '_ > > { tcx . explicit_item_self_bounds (def_id) . map_bound (| bounds | { tcx . mk_clauses_from_iter (util :: elaborate (tcx , bounds . iter () . map (| & (bound , _span) | bound)) . filter_only_self () ,) }) }
}

macro_rules! item_non_self_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function item_non_self_bounds in module {}", module_path!());
    };
}

mkfn!{
    item_non_self_bounds_introspect!();
    # [doc = " This exists as an optimization to compute only the item bounds of the item"] # [doc = " that are not `Self` bounds."] pub (super) fn item_non_self_bounds (tcx : TyCtxt < '_ > , def_id : DefId ,) -> ty :: EarlyBinder < '_ , ty :: Clauses < '_ > > { let all_bounds : FxIndexSet < _ > = tcx . item_bounds (def_id) . skip_binder () . iter () . collect () ; let own_bounds : FxIndexSet < _ > = tcx . item_self_bounds (def_id) . skip_binder () . iter () . collect () ; if all_bounds . len () == own_bounds . len () { ty :: EarlyBinder :: bind (ty :: ListWithCachedTypeInfo :: empty ()) } else { ty :: EarlyBinder :: bind (tcx . mk_clauses_from_iter (all_bounds . difference (& own_bounds) . copied ())) } }
}

macro_rules! impl_super_outlives_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function impl_super_outlives in module {}", module_path!());
    };
}

mkfn!{
    impl_super_outlives_introspect!();
    # [doc = " This exists as an optimization to compute only the supertraits of this impl's"] # [doc = " trait that are outlives bounds."] pub (super) fn impl_super_outlives (tcx : TyCtxt < '_ > , def_id : DefId ,) -> ty :: EarlyBinder < '_ , ty :: Clauses < '_ > > { tcx . impl_trait_header (def_id) . expect ("expected an impl of trait") . trait_ref . map_bound (| trait_ref | { let clause : ty :: Clause < '_ > = trait_ref . upcast (tcx) ; tcx . mk_clauses_from_iter (util :: elaborate (tcx , [clause]) . filter (| clause | { matches ! (clause . kind () . skip_binder () , ty :: ClauseKind :: TypeOutlives (_) | ty :: ClauseKind :: RegionOutlives (_)) })) } ,) }
}
mkitem!{mkstruct!{struct AssocTyToOpaque < 'tcx > { tcx : TyCtxt < 'tcx > , fn_def_id : DefId , }}}
mkitem!{mkimpl!{impl < 'tcx > TypeFolder < TyCtxt < 'tcx > > for AssocTyToOpaque < 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn fold_ty (& mut self , ty : Ty < 'tcx >) -> Ty < 'tcx > { if let ty :: Alias (ty :: Projection , projection_ty) = ty . kind () && let Some (ty :: ImplTraitInTraitData :: Trait { fn_def_id , .. }) = self . tcx . opt_rpitit_info (projection_ty . def_id) && fn_def_id == self . fn_def_id { self . tcx . type_of (projection_ty . def_id) . instantiate (self . tcx , projection_ty . args) } else { ty . super_fold_with (self) } } }}}