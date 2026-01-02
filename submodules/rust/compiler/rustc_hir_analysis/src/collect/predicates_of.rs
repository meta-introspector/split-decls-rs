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
mkuse!{use hir :: Node ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexSet ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_hir :: find_attr ;}
mkuse!{use rustc_middle :: ty :: { self , GenericPredicates , ImplTraitInTraitData , Ty , TyCtxt , TypeVisitable , TypeVisitor , Upcast , } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_span :: { DUMMY_SP , Ident , Span } ;}
mkuse!{use tracing :: { debug , instrument , trace } ;}
mkuse!{use super :: item_bounds :: explicit_item_bounds_with_filter ;}
mkuse!{use crate :: collect :: ItemCtxt ;}
mkuse!{use crate :: constrained_generic_params as cgp ;}
mkuse!{use crate :: delegation :: inherit_predicates_for_delegation_item ;}
mkuse!{use crate :: hir_ty_lowering :: { HirTyLowerer , PredicateFilter , RegionInferReason } ;}

macro_rules! predicates_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function predicates_of in module {}", module_path!());
    };
}

mkfn!{
    predicates_of_introspect!();
    #[doc = " Returns a list of all type predicates (explicit and implicit) for the definition with"] #[doc = " ID `def_id`. This includes all predicates returned by `explicit_predicates_of`, plus"] #[doc = " inferred constraints concerning which regions outlive other regions."] #[instrument (level = "debug" , skip (tcx))] pub (super) fn predicates_of (tcx : TyCtxt < '_ > , def_id : DefId) -> ty :: GenericPredicates < '_ > { let mut result = tcx . explicit_predicates_of (def_id) ; debug ! ("predicates_of: explicit_predicates_of({:?}) = {:?}" , def_id , result) ; let inferred_outlives = tcx . inferred_outlives_of (def_id) ; if ! inferred_outlives . is_empty () { debug ! ("predicates_of: inferred_outlives_of({:?}) = {:?}" , def_id , inferred_outlives ,) ; let inferred_outlives_iter = inferred_outlives . iter () . map (| (clause , span) | ((* clause) . upcast (tcx) , * span)) ; if result . predicates . is_empty () { result . predicates = tcx . arena . alloc_from_iter (inferred_outlives_iter) ; } else { result . predicates = tcx . arena . alloc_from_iter (result . predicates . into_iter () . copied () . chain (inferred_outlives_iter) ,) ; } } if tcx . is_trait (def_id) { let span = DUMMY_SP ; result . predicates = tcx . arena . alloc_from_iter (result . predicates . iter () . copied () . chain (std :: iter :: once ((ty :: TraitRef :: identity (tcx , def_id) . upcast (tcx) , span))) ,) ; } debug ! ("predicates_of({:?}) = {:?}" , def_id , result) ; result }
}

macro_rules! gather_explicit_predicates_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function gather_explicit_predicates_of in module {}", module_path!());
    };
}

mkfn!{
    gather_explicit_predicates_of_introspect!();
    #[doc = " Returns a list of user-specified type predicates for the definition with ID `def_id`."] #[doc = " N.B., this does not include any implied/inferred constraints."] #[instrument (level = "trace" , skip (tcx) , ret)] fn gather_explicit_predicates_of (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> ty :: GenericPredicates < '_ > { use rustc_hir :: * ; match tcx . opt_rpitit_info (def_id . to_def_id ()) { Some (ImplTraitInTraitData :: Trait { fn_def_id , .. }) => { let mut predicates = Vec :: new () ; let identity_args = ty :: GenericArgs :: identity_for_item (tcx , def_id) ; predicates . extend (tcx . explicit_predicates_of (fn_def_id) . instantiate_own (tcx , identity_args)) ; compute_bidirectional_outlives_predicates (tcx , & tcx . generics_of (def_id . to_def_id ()) . own_params [tcx . generics_of (fn_def_id) . own_params . len () ..] , & mut predicates ,) ; return ty :: GenericPredicates { parent : Some (tcx . parent (def_id . to_def_id ())) , predicates : tcx . arena . alloc_from_iter (predicates) , } ; } Some (ImplTraitInTraitData :: Impl { fn_def_id }) => { let trait_item_def_id = tcx . trait_item_of (def_id) . unwrap () ; let trait_assoc_predicates = tcx . explicit_predicates_of (trait_item_def_id) ; let impl_assoc_identity_args = ty :: GenericArgs :: identity_for_item (tcx , def_id) ; let impl_def_id = tcx . parent (fn_def_id) ; let impl_trait_ref_args = tcx . impl_trait_ref (impl_def_id) . unwrap () . instantiate_identity () . args ; let impl_assoc_args = impl_assoc_identity_args . rebase_onto (tcx , impl_def_id , impl_trait_ref_args) ; let impl_predicates = trait_assoc_predicates . instantiate_own (tcx , impl_assoc_args) ; return ty :: GenericPredicates { parent : Some (impl_def_id) , predicates : tcx . arena . alloc_from_iter (impl_predicates) , } ; } None => { } } let hir_id = tcx . local_def_id_to_hir_id (def_id) ; let node = tcx . hir_node (hir_id) ; if let Some (sig) = node . fn_sig () && let Some (sig_id) = sig . decl . opt_delegation_sig_id () { return inherit_predicates_for_delegation_item (tcx , def_id , sig_id) ; } let mut is_trait = None ; let mut is_default_impl_trait = None ; let icx = ItemCtxt :: new (tcx , def_id) ; const NO_GENERICS : & hir :: Generics < '_ > = hir :: Generics :: empty () ; let mut predicates : FxIndexSet < (ty :: Clause < '_ > , Span) > = FxIndexSet :: default () ; let hir_generics = node . generics () . unwrap_or (NO_GENERICS) ; if let Node :: Item (item) = node { match item . kind { ItemKind :: Impl (impl_) => { if let Some (of_trait) = impl_ . of_trait && of_trait . defaultness . is_default () { is_default_impl_trait = tcx . impl_trait_ref (def_id) . map (| t | ty :: Binder :: dummy (t . instantiate_identity ())) ; } } ItemKind :: Trait (_ , _ , _ , _ , _ , self_bounds , ..) | ItemKind :: TraitAlias (_ , _ , self_bounds) => { is_trait = Some ((self_bounds , item . span)) ; } _ => { } } } ; let generics = tcx . generics_of (def_id) ; if let Some ((self_bounds , span)) = is_trait { let mut bounds = Vec :: new () ; icx . lowerer () . lower_bounds (tcx . types . self_param , self_bounds , & mut bounds , ty :: List :: empty () , PredicateFilter :: All ,) ; icx . lowerer () . add_sizedness_bounds (& mut bounds , tcx . types . self_param , self_bounds , None , Some (def_id) , span ,) ; icx . lowerer () . add_default_super_traits (def_id , & mut bounds , self_bounds , hir_generics , span ,) ; predicates . extend (bounds) ; } if let Some (trait_ref) = is_default_impl_trait { predicates . insert ((trait_ref . upcast (tcx) , tcx . def_span (def_id))) ; } for param in hir_generics . params { match param . kind { GenericParamKind :: Lifetime { .. } => () , GenericParamKind :: Type { .. } => { let param_ty = icx . lowerer () . lower_ty_param (param . hir_id) ; let mut bounds = Vec :: new () ; icx . lowerer () . add_sizedness_bounds (& mut bounds , param_ty , & [] , Some ((param . def_id , hir_generics . predicates)) , None , param . span ,) ; icx . lowerer () . add_default_traits (& mut bounds , param_ty , & [] , Some ((param . def_id , hir_generics . predicates)) , param . span ,) ; trace ! (? bounds) ; predicates . extend (bounds) ; trace ! (? predicates) ; } hir :: GenericParamKind :: Const { .. } => { let param_def_id = param . def_id . to_def_id () ; let ct_ty = tcx . type_of (param_def_id) . instantiate_identity () ; let ct = icx . lowerer () . lower_const_param (param_def_id , param . hir_id) ; predicates . insert ((ty :: ClauseKind :: ConstArgHasType (ct , ct_ty) . upcast (tcx) , param . span)) ; } } } trace ! (? predicates) ; for predicate in hir_generics . predicates { match predicate . kind { hir :: WherePredicateKind :: BoundPredicate (bound_pred) => { let ty = icx . lowerer () . lower_ty_maybe_return_type_notation (bound_pred . bounded_ty) ; let bound_vars = tcx . late_bound_vars (predicate . hir_id) ; if bound_pred . bounds . is_empty () { if let ty :: Param (_) = ty . kind () { } else { let span = bound_pred . bounded_ty . span ; let predicate = ty :: Binder :: bind_with_vars (ty :: ClauseKind :: WellFormed (ty . into ()) , bound_vars ,) ; predicates . insert ((predicate . upcast (tcx) , span)) ; } } let mut bounds = Vec :: new () ; icx . lowerer () . lower_bounds (ty , bound_pred . bounds , & mut bounds , bound_vars , PredicateFilter :: All ,) ; predicates . extend (bounds) ; } hir :: WherePredicateKind :: RegionPredicate (region_pred) => { let r1 = icx . lowerer () . lower_lifetime (region_pred . lifetime , RegionInferReason :: RegionPredicate) ; predicates . extend (region_pred . bounds . iter () . map (| bound | { let (r2 , span) = match bound { hir :: GenericBound :: Outlives (lt) => (icx . lowerer () . lower_lifetime (lt , RegionInferReason :: RegionPredicate) , lt . ident . span ,) , bound => { span_bug ! (bound . span () , "lifetime param bounds must be outlives, but found {bound:?}") } } ; let pred = ty :: ClauseKind :: RegionOutlives (ty :: OutlivesPredicate (r1 , r2)) . upcast (tcx) ; (pred , span) })) } hir :: WherePredicateKind :: EqPredicate (..) => { } } } if tcx . features () . generic_const_exprs () { predicates . extend (const_evaluatable_predicates_of (tcx , def_id , & predicates)) ; } let attrs = tcx . hir_attrs (tcx . local_def_id_to_hir_id (def_id)) ; let allow_unstable_feature_attr = find_attr ! (attrs , AttributeKind :: UnstableFeatureBound (i) => i) . map (| i | i . as_slice ()) . unwrap_or_default () ; for (feat_name , span) in allow_unstable_feature_attr { predicates . insert ((ty :: ClauseKind :: UnstableFeature (* feat_name) . upcast (tcx) , * span)) ; } let mut predicates : Vec < _ > = predicates . into_iter () . collect () ; if let Node :: Item (& Item { kind : ItemKind :: Impl { .. } , .. }) = node { let self_ty = tcx . type_of (def_id) . instantiate_identity () ; let trait_ref = tcx . impl_trait_ref (def_id) . map (ty :: EarlyBinder :: instantiate_identity) ; cgp :: setup_constraining_predicates (tcx , & mut predicates , trait_ref , & mut cgp :: parameters_for_impl (tcx , self_ty , trait_ref) ,) ; } if let Node :: OpaqueTy (..) = node { compute_bidirectional_outlives_predicates (tcx , & generics . own_params , & mut predicates) ; debug ! (? predicates) ; } ty :: GenericPredicates { parent : generics . parent , predicates : tcx . arena . alloc_from_iter (predicates) , } }
}

macro_rules! compute_bidirectional_outlives_predicates_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compute_bidirectional_outlives_predicates in module {}", module_path!());
    };
}

mkfn!{
    compute_bidirectional_outlives_predicates_introspect!();
    #[doc = " Opaques have duplicated lifetimes and we need to compute bidirectional outlives predicates to"] #[doc = " enforce that these lifetimes stay in sync."] fn compute_bidirectional_outlives_predicates < 'tcx > (tcx : TyCtxt < 'tcx > , opaque_own_params : & [ty :: GenericParamDef] , predicates : & mut Vec < (ty :: Clause < 'tcx > , Span) > ,) { for param in opaque_own_params { let orig_lifetime = tcx . map_opaque_lifetime_to_parent_lifetime (param . def_id . expect_local ()) ; if let ty :: ReEarlyParam (..) = orig_lifetime . kind () { let dup_lifetime = ty :: Region :: new_early_param (tcx , ty :: EarlyParamRegion { index : param . index , name : param . name } ,) ; let span = tcx . def_span (param . def_id) ; predicates . push ((ty :: ClauseKind :: RegionOutlives (ty :: OutlivesPredicate (orig_lifetime , dup_lifetime)) . upcast (tcx) , span ,)) ; predicates . push ((ty :: ClauseKind :: RegionOutlives (ty :: OutlivesPredicate (dup_lifetime , orig_lifetime)) . upcast (tcx) , span ,)) ; } } }
}

macro_rules! const_evaluatable_predicates_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function const_evaluatable_predicates_of in module {}", module_path!());
    };
}

mkfn!{
    const_evaluatable_predicates_of_introspect!();
    #[instrument (level = "debug" , skip (tcx , predicates) , ret)] fn const_evaluatable_predicates_of < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId , predicates : & FxIndexSet < (ty :: Clause < 'tcx > , Span) > ,) -> FxIndexSet < (ty :: Clause < 'tcx > , Span) > { struct ConstCollector < 'tcx > { tcx : TyCtxt < 'tcx > , preds : FxIndexSet < (ty :: Clause < 'tcx > , Span) > , } fn is_const_param_default (tcx : TyCtxt < '_ > , def : LocalDefId) -> bool { let hir_id = tcx . local_def_id_to_hir_id (def) ; let (_ , parent_node) = tcx . hir_parent_iter (hir_id) . skip_while (| (_ , n) | matches ! (n , Node :: ConstArg (..))) . next () . unwrap () ; matches ! (parent_node , Node :: GenericParam (hir :: GenericParam { kind : hir :: GenericParamKind :: Const { .. } , .. })) } impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for ConstCollector < 'tcx > { fn visit_const (& mut self , c : ty :: Const < 'tcx >) { if let ty :: ConstKind :: Unevaluated (uv) = c . kind () { if let Some (local) = uv . def . as_local () && is_const_param_default (self . tcx , local) { return ; } let span = self . tcx . def_span (uv . def) ; self . preds . insert ((ty :: ClauseKind :: ConstEvaluatable (c) . upcast (self . tcx) , span)) ; } } } let hir_id = tcx . local_def_id_to_hir_id (def_id) ; let node = tcx . hir_node (hir_id) ; let mut collector = ConstCollector { tcx , preds : FxIndexSet :: default () } ; for (clause , _sp) in predicates { clause . visit_with (& mut collector) ; } if let hir :: Node :: Item (item) = node && let hir :: ItemKind :: Impl (_) = item . kind { if let Some (of_trait) = tcx . impl_trait_ref (def_id) { debug ! ("visit impl trait_ref") ; of_trait . instantiate_identity () . visit_with (& mut collector) ; } debug ! ("visit self_ty") ; let self_ty = tcx . type_of (def_id) ; self_ty . instantiate_identity () . visit_with (& mut collector) ; } if let Some (_) = tcx . hir_fn_sig_by_hir_id (hir_id) { debug ! ("visit fn sig") ; let fn_sig = tcx . fn_sig (def_id) ; let fn_sig = fn_sig . instantiate_identity () ; debug ! (? fn_sig) ; fn_sig . visit_with (& mut collector) ; } collector . preds }
}

macro_rules! trait_explicit_predicates_and_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trait_explicit_predicates_and_bounds in module {}", module_path!());
    };
}

mkfn!{
    trait_explicit_predicates_and_bounds_introspect!();
    pub (super) fn trait_explicit_predicates_and_bounds (tcx : TyCtxt < '_ > , def_id : LocalDefId ,) -> ty :: GenericPredicates < '_ > { assert_eq ! (tcx . def_kind (def_id) , DefKind :: Trait) ; gather_explicit_predicates_of (tcx , def_id) }
}

macro_rules! explicit_predicates_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function explicit_predicates_of in module {}", module_path!());
    };
}

mkfn!{
    explicit_predicates_of_introspect!();
    pub (super) fn explicit_predicates_of < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId ,) -> ty :: GenericPredicates < 'tcx > { let def_kind = tcx . def_kind (def_id) ; if let DefKind :: Trait = def_kind { let predicates_and_bounds = tcx . trait_explicit_predicates_and_bounds (def_id) ; let trait_identity_args = ty :: GenericArgs :: identity_for_item (tcx , def_id) ; let is_assoc_item_ty = | ty : Ty < 'tcx > | { if let ty :: Alias (ty :: Projection , projection) = ty . kind () { projection . args == trait_identity_args && ! tcx . is_impl_trait_in_trait (projection . def_id) && tcx . parent (projection . def_id) == def_id . to_def_id () } else { false } } ; let predicates : Vec < _ > = predicates_and_bounds . predicates . iter () . copied () . filter (| (pred , _) | match pred . kind () . skip_binder () { ty :: ClauseKind :: Trait (tr) => ! is_assoc_item_ty (tr . self_ty ()) , ty :: ClauseKind :: Projection (proj) => { ! is_assoc_item_ty (proj . projection_term . self_ty ()) } ty :: ClauseKind :: TypeOutlives (outlives) => ! is_assoc_item_ty (outlives . 0) , _ => true , }) . collect () ; if predicates . len () == predicates_and_bounds . predicates . len () { predicates_and_bounds } else { ty :: GenericPredicates { parent : predicates_and_bounds . parent , predicates : tcx . arena . alloc_slice (& predicates) , } } } else { if matches ! (def_kind , DefKind :: AnonConst) && tcx . features () . generic_const_exprs () && let Some (defaulted_param_def_id) = tcx . hir_opt_const_param_default_param_def_id (tcx . local_def_id_to_hir_id (def_id)) { let parent_def_id = tcx . local_parent (def_id) ; let parent_preds = tcx . explicit_predicates_of (parent_def_id) ; let filtered_predicates = parent_preds . predicates . into_iter () . filter (| (pred , _) | { if let ty :: ClauseKind :: ConstArgHasType (ct , _) = pred . kind () . skip_binder () { match ct . kind () { ty :: ConstKind :: Param (param_const) => { let defaulted_param_idx = tcx . generics_of (parent_def_id) . param_def_id_to_index [& defaulted_param_def_id . to_def_id ()] ; param_const . index < defaulted_param_idx } _ => bug ! ("`ConstArgHasType` in `predicates_of`\
                                 that isn't a `Param` const") , } } else { true } }) . cloned () ; return GenericPredicates { parent : parent_preds . parent , predicates : { tcx . arena . alloc_from_iter (filtered_predicates) } , } ; } gather_explicit_predicates_of (tcx , def_id) } }
}

macro_rules! explicit_super_predicates_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function explicit_super_predicates_of in module {}", module_path!());
    };
}

mkfn!{
    explicit_super_predicates_of_introspect!();
    #[doc = " Ensures that the super-predicates of the trait with a `DefId`"] #[doc = " of `trait_def_id` are lowered and stored. This also ensures that"] #[doc = " the transitive super-predicates are lowered."] pub (super) fn explicit_super_predicates_of < 'tcx > (tcx : TyCtxt < 'tcx > , trait_def_id : LocalDefId ,) -> ty :: EarlyBinder < 'tcx , & 'tcx [(ty :: Clause < 'tcx > , Span)] > { implied_predicates_with_filter (tcx , trait_def_id . to_def_id () , PredicateFilter :: SelfOnly) }
}

macro_rules! explicit_supertraits_containing_assoc_item_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function explicit_supertraits_containing_assoc_item in module {}", module_path!());
    };
}

mkfn!{
    explicit_supertraits_containing_assoc_item_introspect!();
    pub (super) fn explicit_supertraits_containing_assoc_item < 'tcx > (tcx : TyCtxt < 'tcx > , (trait_def_id , assoc_ident) : (DefId , Ident) ,) -> ty :: EarlyBinder < 'tcx , & 'tcx [(ty :: Clause < 'tcx > , Span)] > { implied_predicates_with_filter (tcx , trait_def_id , PredicateFilter :: SelfTraitThatDefines (assoc_ident) ,) }
}

macro_rules! explicit_implied_predicates_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function explicit_implied_predicates_of in module {}", module_path!());
    };
}

mkfn!{
    explicit_implied_predicates_of_introspect!();
    pub (super) fn explicit_implied_predicates_of < 'tcx > (tcx : TyCtxt < 'tcx > , trait_def_id : LocalDefId ,) -> ty :: EarlyBinder < 'tcx , & 'tcx [(ty :: Clause < 'tcx > , Span)] > { implied_predicates_with_filter (tcx , trait_def_id . to_def_id () , if tcx . is_trait_alias (trait_def_id . to_def_id ()) { PredicateFilter :: All } else { PredicateFilter :: SelfAndAssociatedTypeBounds } ,) }
}

macro_rules! implied_predicates_with_filter_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function implied_predicates_with_filter in module {}", module_path!());
    };
}

mkfn!{
    implied_predicates_with_filter_introspect!();
    #[doc = " Ensures that the super-predicates of the trait with a `DefId`"] #[doc = " of `trait_def_id` are lowered and stored. This also ensures that"] #[doc = " the transitive super-predicates are lowered."] pub (super) fn implied_predicates_with_filter < 'tcx > (tcx : TyCtxt < 'tcx > , trait_def_id : DefId , filter : PredicateFilter ,) -> ty :: EarlyBinder < 'tcx , & 'tcx [(ty :: Clause < 'tcx > , Span)] > { let Some (trait_def_id) = trait_def_id . as_local () else { assert_matches ! (filter , PredicateFilter :: SelfTraitThatDefines (_)) ; return tcx . explicit_super_predicates_of (trait_def_id) ; } ; let Node :: Item (item) = tcx . hir_node_by_def_id (trait_def_id) else { bug ! ("trait_def_id {trait_def_id:?} is not an item") ; } ; let (generics , superbounds) = match item . kind { hir :: ItemKind :: Trait (.. , generics , supertraits , _) => (generics , supertraits) , hir :: ItemKind :: TraitAlias (_ , generics , supertraits) => (generics , supertraits) , _ => span_bug ! (item . span , "super_predicates invoked on non-trait") , } ; let icx = ItemCtxt :: new (tcx , trait_def_id) ; let self_param_ty = tcx . types . self_param ; let mut bounds = Vec :: new () ; icx . lowerer () . lower_bounds (self_param_ty , superbounds , & mut bounds , ty :: List :: empty () , filter) ; match filter { PredicateFilter :: All | PredicateFilter :: SelfOnly | PredicateFilter :: SelfTraitThatDefines (_) | PredicateFilter :: SelfAndAssociatedTypeBounds => { icx . lowerer () . add_default_super_traits (trait_def_id , & mut bounds , superbounds , generics , item . span ,) ; } PredicateFilter :: ConstIfConst | PredicateFilter :: SelfConstIfConst => { } } let where_bounds_that_match = icx . probe_ty_param_bounds_in_generics (generics , item . owner_id . def_id , filter) ; let implied_bounds = & * tcx . arena . alloc_from_iter (bounds . into_iter () . chain (where_bounds_that_match)) ; debug ! (? implied_bounds) ; match filter { PredicateFilter :: SelfOnly => { for & (pred , span) in implied_bounds { debug ! ("superbound: {:?}" , pred) ; if let ty :: ClauseKind :: Trait (bound) = pred . kind () . skip_binder () && bound . polarity == ty :: PredicatePolarity :: Positive { tcx . at (span) . explicit_super_predicates_of (bound . def_id ()) ; } } } PredicateFilter :: All | PredicateFilter :: SelfAndAssociatedTypeBounds => { for & (pred , span) in implied_bounds { debug ! ("superbound: {:?}" , pred) ; if let ty :: ClauseKind :: Trait (bound) = pred . kind () . skip_binder () && bound . polarity == ty :: PredicatePolarity :: Positive { tcx . at (span) . explicit_implied_predicates_of (bound . def_id ()) ; } } } _ => { } } assert_only_contains_predicates_from (filter , implied_bounds , tcx . types . self_param) ; ty :: EarlyBinder :: bind (implied_bounds) }
}

macro_rules! assert_only_contains_predicates_from_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function assert_only_contains_predicates_from in module {}", module_path!());
    };
}

mkfn!{
    assert_only_contains_predicates_from_introspect!();
    pub (super) fn assert_only_contains_predicates_from < 'tcx > (filter : PredicateFilter , bounds : & 'tcx [(ty :: Clause < 'tcx > , Span)] , ty : Ty < 'tcx > ,) { if ! cfg ! (debug_assertions) { return ; } match filter { PredicateFilter :: SelfOnly => { for (clause , _) in bounds { match clause . kind () . skip_binder () { ty :: ClauseKind :: Trait (trait_predicate) => { assert_eq ! (trait_predicate . self_ty () , ty , "expected `Self` predicate when computing \
                            `{filter:?}` implied bounds: {clause:?}") ; } ty :: ClauseKind :: Projection (projection_predicate) => { assert_eq ! (projection_predicate . self_ty () , ty , "expected `Self` predicate when computing \
                            `{filter:?}` implied bounds: {clause:?}") ; } ty :: ClauseKind :: TypeOutlives (outlives_predicate) => { assert_eq ! (outlives_predicate . 0 , ty , "expected `Self` predicate when computing \
                            `{filter:?}` implied bounds: {clause:?}") ; } ty :: ClauseKind :: HostEffect (host_effect_predicate) => { assert_eq ! (host_effect_predicate . self_ty () , ty , "expected `Self` predicate when computing \
                            `{filter:?}` implied bounds: {clause:?}") ; } ty :: ClauseKind :: RegionOutlives (_) | ty :: ClauseKind :: ConstArgHasType (_ , _) | ty :: ClauseKind :: WellFormed (_) | ty :: ClauseKind :: UnstableFeature (_) | ty :: ClauseKind :: ConstEvaluatable (_) => { bug ! ("unexpected non-`Self` predicate when computing \
                            `{filter:?}` implied bounds: {clause:?}") ; } } } } PredicateFilter :: SelfTraitThatDefines (_) => { for (clause , _) in bounds { match clause . kind () . skip_binder () { ty :: ClauseKind :: Trait (trait_predicate) => { assert_eq ! (trait_predicate . self_ty () , ty , "expected `Self` predicate when computing \
                            `{filter:?}` implied bounds: {clause:?}") ; } ty :: ClauseKind :: Projection (_) | ty :: ClauseKind :: TypeOutlives (_) | ty :: ClauseKind :: RegionOutlives (_) | ty :: ClauseKind :: ConstArgHasType (_ , _) | ty :: ClauseKind :: WellFormed (_) | ty :: ClauseKind :: ConstEvaluatable (_) | ty :: ClauseKind :: UnstableFeature (_) | ty :: ClauseKind :: HostEffect (..) => { bug ! ("unexpected non-`Self` predicate when computing \
                            `{filter:?}` implied bounds: {clause:?}") ; } } } } PredicateFilter :: ConstIfConst => { for (clause , _) in bounds { match clause . kind () . skip_binder () { ty :: ClauseKind :: HostEffect (ty :: HostEffectPredicate { trait_ref : _ , constness : ty :: BoundConstness :: Maybe , }) => { } _ => { bug ! ("unexpected non-`HostEffect` predicate when computing \
                            `{filter:?}` implied bounds: {clause:?}") ; } } } } PredicateFilter :: SelfConstIfConst => { for (clause , _) in bounds { match clause . kind () . skip_binder () { ty :: ClauseKind :: HostEffect (pred) => { assert_eq ! (pred . constness , ty :: BoundConstness :: Maybe , "expected `[const]` predicate when computing `{filter:?}` \
                            implied bounds: {clause:?}" ,) ; assert_eq ! (pred . trait_ref . self_ty () , ty , "expected `Self` predicate when computing `{filter:?}` \
                            implied bounds: {clause:?}") ; } _ => { bug ! ("unexpected non-`HostEffect` predicate when computing \
                            `{filter:?}` implied bounds: {clause:?}") ; } } } } PredicateFilter :: All | PredicateFilter :: SelfAndAssociatedTypeBounds => { } } }
}

macro_rules! type_param_predicates_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_param_predicates in module {}", module_path!());
    };
}

mkfn!{
    type_param_predicates_introspect!();
    #[doc = " Returns the predicates defined on `item_def_id` of the form"] #[doc = " `X: Foo` where `X` is the type parameter `def_id`."] #[instrument (level = "trace" , skip (tcx))] pub (super) fn type_param_predicates < 'tcx > (tcx : TyCtxt < 'tcx > , (item_def_id , def_id , assoc_ident) : (LocalDefId , LocalDefId , Ident) ,) -> ty :: EarlyBinder < 'tcx , & 'tcx [(ty :: Clause < 'tcx > , Span)] > { match tcx . opt_rpitit_info (item_def_id . to_def_id ()) { Some (ty :: ImplTraitInTraitData :: Trait { opaque_def_id , .. }) => { return tcx . type_param_predicates ((opaque_def_id . expect_local () , def_id , assoc_ident)) ; } Some (ty :: ImplTraitInTraitData :: Impl { .. }) => { unreachable ! ("should not be lowering bounds on RPITIT in impl") } None => { } } let param_id = tcx . local_def_id_to_hir_id (def_id) ; let param_owner = tcx . hir_ty_param_owner (def_id) ; let parent = if item_def_id == param_owner { None } else { tcx . generics_of (item_def_id) . parent . map (| def_id | def_id . expect_local ()) } ; let result = if let Some (parent) = parent { let icx = ItemCtxt :: new (tcx , parent) ; icx . probe_ty_param_bounds (DUMMY_SP , def_id , assoc_ident) } else { ty :: EarlyBinder :: bind (& [] as & [_]) } ; let mut extend = None ; let item_hir_id = tcx . local_def_id_to_hir_id (item_def_id) ; let hir_node = tcx . hir_node (item_hir_id) ; let Some (hir_generics) = hir_node . generics () else { return result ; } ; if let Node :: Item (item) = hir_node && let hir :: ItemKind :: Trait (..) = item . kind && param_id == item_hir_id { let identity_trait_ref = ty :: TraitRef :: identity (tcx , item_def_id . to_def_id ()) ; extend = Some ((identity_trait_ref . upcast (tcx) , item . span)) ; } let icx = ItemCtxt :: new (tcx , item_def_id) ; let extra_predicates = extend . into_iter () . chain (icx . probe_ty_param_bounds_in_generics (hir_generics , def_id , PredicateFilter :: SelfTraitThatDefines (assoc_ident) ,)) ; let bounds = & * tcx . arena . alloc_from_iter (result . skip_binder () . iter () . copied () . chain (extra_predicates)) ; let self_ty = match tcx . def_kind (def_id) { DefKind :: TyParam => Ty :: new_param (tcx , tcx . generics_of (item_def_id) . param_def_id_to_index (tcx , def_id . to_def_id ()) . expect ("expected generic param to be owned by item") , tcx . item_name (def_id . to_def_id ()) ,) , DefKind :: Trait | DefKind :: TraitAlias => tcx . types . self_param , _ => unreachable ! () , } ; assert_only_contains_predicates_from (PredicateFilter :: SelfTraitThatDefines (assoc_ident) , bounds , self_ty ,) ; ty :: EarlyBinder :: bind (bounds) }
}
mkitem!{mkimpl!{impl < 'tcx > ItemCtxt < 'tcx > { #[doc = " Finds bounds from `hir::Generics`."] #[doc = ""] #[doc = " This requires scanning through the HIR."] #[doc = " We do this to avoid having to lower *all* the bounds, which would create artificial cycles."] #[doc = " Instead, we can only lower the bounds for a type parameter `X` if `X::Foo` is used."] #[instrument (level = "trace" , skip (self , hir_generics))] fn probe_ty_param_bounds_in_generics (& self , hir_generics : & 'tcx hir :: Generics < 'tcx > , param_def_id : LocalDefId , filter : PredicateFilter ,) -> Vec < (ty :: Clause < 'tcx > , Span) > { let mut bounds = Vec :: new () ; for predicate in hir_generics . predicates { let hir_id = predicate . hir_id ; let hir :: WherePredicateKind :: BoundPredicate (predicate) = predicate . kind else { continue ; } ; match filter { _ if predicate . is_param_bound (param_def_id . to_def_id ()) => { } PredicateFilter :: All => { } PredicateFilter :: SelfOnly | PredicateFilter :: SelfTraitThatDefines (_) | PredicateFilter :: SelfConstIfConst | PredicateFilter :: SelfAndAssociatedTypeBounds => continue , PredicateFilter :: ConstIfConst => unreachable ! () , } let bound_ty = self . lowerer () . lower_ty_maybe_return_type_notation (predicate . bounded_ty) ; let bound_vars = self . tcx . late_bound_vars (hir_id) ; self . lowerer () . lower_bounds (bound_ty , predicate . bounds , & mut bounds , bound_vars , filter ,) ; } bounds } }}}

macro_rules! const_conditions_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function const_conditions in module {}", module_path!());
    };
}

mkfn!{
    const_conditions_introspect!();
    pub (super) fn const_conditions < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId ,) -> ty :: ConstConditions < 'tcx > { if ! tcx . is_conditionally_const (def_id) { bug ! ("const_conditions invoked for item that is not conditionally const: {def_id:?}") ; } match tcx . opt_rpitit_info (def_id . to_def_id ()) { Some (ty :: ImplTraitInTraitData :: Impl { fn_def_id } | ty :: ImplTraitInTraitData :: Trait { fn_def_id , .. } ,) => return tcx . const_conditions (fn_def_id) , None => { } } let (generics , trait_def_id_and_supertraits , has_parent) = match tcx . hir_node_by_def_id (def_id) { Node :: Item (item) => match item . kind { hir :: ItemKind :: Impl (impl_) => (impl_ . generics , None , false) , hir :: ItemKind :: Fn { generics , .. } => (generics , None , false) , hir :: ItemKind :: Trait (_ , _ , _ , _ , generics , supertraits , _) => { (generics , Some ((item . owner_id . def_id , supertraits)) , false) } _ => bug ! ("const_conditions called on wrong item: {def_id:?}") , } , Node :: TraitItem (item) => match item . kind { hir :: TraitItemKind :: Fn (_ , _) | hir :: TraitItemKind :: Type (_ , _) => { (item . generics , None , true) } _ => bug ! ("const_conditions called on wrong item: {def_id:?}") , } , Node :: ImplItem (item) => match item . kind { hir :: ImplItemKind :: Fn (_ , _) | hir :: ImplItemKind :: Type (_) => { (item . generics , None , tcx . is_conditionally_const (tcx . local_parent (def_id))) } _ => bug ! ("const_conditions called on wrong item: {def_id:?}") , } , Node :: ForeignItem (item) => match item . kind { hir :: ForeignItemKind :: Fn (_ , _ , generics) => (generics , None , false) , _ => bug ! ("const_conditions called on wrong item: {def_id:?}") , } , Node :: OpaqueTy (opaque) => match opaque . origin { hir :: OpaqueTyOrigin :: FnReturn { parent , .. } => return tcx . const_conditions (parent) , hir :: OpaqueTyOrigin :: AsyncFn { .. } | hir :: OpaqueTyOrigin :: TyAlias { .. } => { unreachable ! () } } , Node :: Ctor (hir :: VariantData :: Tuple { .. }) => return Default :: default () , _ => bug ! ("const_conditions called on wrong item: {def_id:?}") , } ; let icx = ItemCtxt :: new (tcx , def_id) ; let mut bounds = Vec :: new () ; for pred in generics . predicates { match pred . kind { hir :: WherePredicateKind :: BoundPredicate (bound_pred) => { let ty = icx . lowerer () . lower_ty_maybe_return_type_notation (bound_pred . bounded_ty) ; let bound_vars = tcx . late_bound_vars (pred . hir_id) ; icx . lowerer () . lower_bounds (ty , bound_pred . bounds . iter () , & mut bounds , bound_vars , PredicateFilter :: ConstIfConst ,) ; } _ => { } } } if let Some ((def_id , supertraits)) = trait_def_id_and_supertraits { bounds . push ((ty :: Binder :: dummy (ty :: TraitRef :: identity (tcx , def_id . to_def_id ())) . to_host_effect_clause (tcx , ty :: BoundConstness :: Maybe) , DUMMY_SP ,)) ; icx . lowerer () . lower_bounds (tcx . types . self_param , supertraits , & mut bounds , ty :: List :: empty () , PredicateFilter :: ConstIfConst ,) ; } ty :: ConstConditions { parent : has_parent . then (| | tcx . local_parent (def_id) . to_def_id ()) , predicates : tcx . arena . alloc_from_iter (bounds . into_iter () . map (| (clause , span) | { (clause . kind () . map_bound (| clause | match clause { ty :: ClauseKind :: HostEffect (ty :: HostEffectPredicate { trait_ref , constness : ty :: BoundConstness :: Maybe , }) => trait_ref , _ => bug ! ("converted {clause:?}") , }) , span ,) })) , } }
}

macro_rules! explicit_implied_const_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function explicit_implied_const_bounds in module {}", module_path!());
    };
}

mkfn!{
    explicit_implied_const_bounds_introspect!();
    pub (super) fn explicit_implied_const_bounds < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId ,) -> ty :: EarlyBinder < 'tcx , & 'tcx [(ty :: PolyTraitRef < 'tcx > , Span)] > { if ! tcx . is_conditionally_const (def_id) { bug ! ("explicit_implied_const_bounds invoked for item that is not conditionally const: {def_id:?}") ; } let bounds = match tcx . opt_rpitit_info (def_id . to_def_id ()) { Some (ty :: ImplTraitInTraitData :: Trait { .. }) => { explicit_item_bounds_with_filter (tcx , def_id , PredicateFilter :: ConstIfConst) } Some (ty :: ImplTraitInTraitData :: Impl { .. }) => { span_bug ! (tcx . def_span (def_id) , "RPITIT in impl should not have item bounds") } None => match tcx . hir_node_by_def_id (def_id) { Node :: Item (hir :: Item { kind : hir :: ItemKind :: Trait (..) , .. }) => { implied_predicates_with_filter (tcx , def_id . to_def_id () , PredicateFilter :: SelfConstIfConst ,) } Node :: TraitItem (hir :: TraitItem { kind : hir :: TraitItemKind :: Type (..) , .. }) | Node :: OpaqueTy (_) => { explicit_item_bounds_with_filter (tcx , def_id , PredicateFilter :: ConstIfConst) } _ => bug ! ("explicit_implied_const_bounds called on wrong item: {def_id:?}") , } , } ; bounds . map_bound (| bounds | { & * tcx . arena . alloc_from_iter (bounds . iter () . copied () . map (| (clause , span) | { (clause . kind () . map_bound (| clause | match clause { ty :: ClauseKind :: HostEffect (ty :: HostEffectPredicate { trait_ref , constness : ty :: BoundConstness :: Maybe , }) => trait_ref , _ => bug ! ("converted {clause:?}") , }) , span ,) })) }) }
}