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
mkuse!{use std :: marker :: PhantomData ;}
mkuse!{use smallvec :: smallvec ;}
mkuse!{use crate :: data_structures :: HashSet ;}
mkuse!{use crate :: inherent :: * ;}
mkuse!{use crate :: lang_items :: SolverTraitLangItem ;}
mkuse!{use crate :: outlives :: { Component , push_outlives_components } ;}
mkuse!{use crate :: { self as ty , Interner , Upcast as _ } ;}
mkitem!{mkstruct!{# [doc = " \"Elaboration\" is the process of identifying all the predicates that"] # [doc = " are implied by a source predicate. Currently, this basically means"] # [doc = " walking the \"supertraits\" and other similar assumptions. For example,"] # [doc = " if we know that `T: Ord`, the elaborator would deduce that `T: PartialOrd`"] # [doc = " holds as well. Similarly, if we have `trait Foo: 'static`, and we know that"] # [doc = " `T: Foo`, then we know that `T: 'static`."] pub struct Elaborator < I : Interner , O > { cx : I , stack : Vec < O > , visited : HashSet < ty :: Binder < I , ty :: PredicateKind < I > > > , mode : Filter , elaborate_sized : ElaborateSized , }}}
mkitem!{mkenum!{enum Filter { All , OnlySelf , }}}
mkitem!{mkenum!{# [derive (Eq , PartialEq)] enum ElaborateSized { Yes , No , }}}
mkitem!{mktrait!{# [doc = " Describes how to elaborate an obligation into a sub-obligation."] pub trait Elaboratable < I : Interner > { fn predicate (& self) -> I :: Predicate ; fn child (& self , clause : I :: Clause) -> Self ; fn child_with_derived_cause (& self , clause : I :: Clause , span : I :: Span , parent_trait_pred : ty :: Binder < I , ty :: TraitPredicate < I > > , index : usize ,) -> Self ; }}}
mkitem!{mkstruct!{pub struct ClauseWithSupertraitSpan < I : Interner > { pub clause : I :: Clause , pub supertrait_span : I :: Span , }}}
mkitem!{mkimpl!{impl < I : Interner > ClauseWithSupertraitSpan < I > { pub fn new (clause : I :: Clause , span : I :: Span) -> Self { ClauseWithSupertraitSpan { clause , supertrait_span : span } } }}}
mkitem!{mkimpl!{impl < I : Interner > Elaboratable < I > for ClauseWithSupertraitSpan < I > { fn predicate (& self) -> < I as Interner > :: Predicate { self . clause . as_predicate () } fn child (& self , clause : < I as Interner > :: Clause) -> Self { ClauseWithSupertraitSpan { clause , supertrait_span : self . supertrait_span } } fn child_with_derived_cause (& self , clause : < I as Interner > :: Clause , supertrait_span : < I as Interner > :: Span , _parent_trait_pred : crate :: Binder < I , crate :: TraitPredicate < I > > , _index : usize ,) -> Self { ClauseWithSupertraitSpan { clause , supertrait_span } } }}}

macro_rules! elaborate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function elaborate in module {}", module_path!());
    };
}

mkfn!{
    elaborate_introspect!();
    pub fn elaborate < I : Interner , O : Elaboratable < I > > (cx : I , obligations : impl IntoIterator < Item = O > ,) -> Elaborator < I , O > { let mut elaborator = Elaborator { cx , stack : Vec :: new () , visited : HashSet :: default () , mode : Filter :: All , elaborate_sized : ElaborateSized :: No , } ; elaborator . extend_deduped (obligations) ; elaborator }
}
mkitem!{mkimpl!{impl < I : Interner , O : Elaboratable < I > > Elaborator < I , O > { # [doc = " Adds `obligations` to the stack."] fn extend_deduped (& mut self , obligations : impl IntoIterator < Item = O >) { self . stack . extend (obligations . into_iter () . filter (| o | { self . visited . insert (self . cx . anonymize_bound_vars (o . predicate () . kind ())) }) ,) ; } # [doc = " Filter to only the supertraits of trait predicates, i.e. only the predicates"] # [doc = " that have `Self` as their self type, instead of all implied predicates."] pub fn filter_only_self (mut self) -> Self { self . mode = Filter :: OnlySelf ; self } # [doc = " Start elaborating `Sized` - reqd during coherence checking, normally skipped to improve"] # [doc = " compiler performance."] pub fn elaborate_sized (mut self) -> Self { self . elaborate_sized = ElaborateSized :: Yes ; self } fn elaborate (& mut self , elaboratable : & O) { let cx = self . cx ; let Some (clause) = elaboratable . predicate () . as_clause () else { return ; } ; if self . elaborate_sized == ElaborateSized :: No && let Some (did) = clause . as_trait_clause () . map (| c | c . def_id ()) && self . cx . is_trait_lang_item (did , SolverTraitLangItem :: Sized) { return ; } let bound_clause = clause . kind () ; match bound_clause . skip_binder () { ty :: ClauseKind :: Trait (data) => { if data . polarity != ty :: PredicatePolarity :: Positive { return ; } let map_to_child_clause = | (index , (clause , span)) : (usize , (I :: Clause , I :: Span)) | { elaboratable . child_with_derived_cause (clause . instantiate_supertrait (cx , bound_clause . rebind (data . trait_ref)) , span , bound_clause . rebind (data) , index ,) } ; match self . mode { Filter :: All => self . extend_deduped (cx . explicit_implied_predicates_of (data . def_id () . into ()) . iter_identity () . enumerate () . map (map_to_child_clause) ,) , Filter :: OnlySelf => self . extend_deduped (cx . explicit_super_predicates_of (data . def_id ()) . iter_identity () . enumerate () . map (map_to_child_clause) ,) , } ; } ty :: ClauseKind :: HostEffect (data) => self . extend_deduped (cx . explicit_implied_const_bounds (data . def_id () . into ()) . iter_identity () . map (| trait_ref | { elaboratable . child (trait_ref . to_host_effect_clause (cx , data . constness) . instantiate_supertrait (cx , bound_clause . rebind (data . trait_ref)) ,) } ,) ,) , ty :: ClauseKind :: TypeOutlives (ty :: OutlivesPredicate (ty_max , r_min)) => { if r_min . is_bound () { return ; } let mut components = smallvec ! [] ; push_outlives_components (cx , ty_max , & mut components) ; self . extend_deduped (components . into_iter () . filter_map (| component | elaborate_component_to_clause (cx , component , r_min)) . map (| clause | elaboratable . child (bound_clause . rebind (clause) . upcast (cx))) ,) ; } ty :: ClauseKind :: RegionOutlives (..) => { } ty :: ClauseKind :: WellFormed (..) => { } ty :: ClauseKind :: Projection (..) => { } ty :: ClauseKind :: ConstEvaluatable (..) => { } ty :: ClauseKind :: ConstArgHasType (..) => { } ty :: ClauseKind :: UnstableFeature (_) => { } } } }}}

macro_rules! elaborate_component_to_clause_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function elaborate_component_to_clause in module {}", module_path!());
    };
}

mkfn!{
    elaborate_component_to_clause_introspect!();
    fn elaborate_component_to_clause < I : Interner > (cx : I , component : Component < I > , outlives_region : I :: Region ,) -> Option < ty :: ClauseKind < I > > { match component { Component :: Region (r) => { if r . is_bound () { None } else { Some (ty :: ClauseKind :: RegionOutlives (ty :: OutlivesPredicate (r , outlives_region))) } } Component :: Param (p) => { let ty = Ty :: new_param (cx , p) ; Some (ty :: ClauseKind :: TypeOutlives (ty :: OutlivesPredicate (ty , outlives_region))) } Component :: Placeholder (p) => { let ty = Ty :: new_placeholder (cx , p) ; Some (ty :: ClauseKind :: TypeOutlives (ty :: OutlivesPredicate (ty , outlives_region))) } Component :: UnresolvedInferenceVariable (_) => None , Component :: Alias (alias_ty) => { Some (ty :: ClauseKind :: TypeOutlives (ty :: OutlivesPredicate (alias_ty . to_ty (cx) , outlives_region ,))) } Component :: EscapingAlias (_) => { None } } }
}
mkitem!{mkimpl!{impl < I : Interner , O : Elaboratable < I > > Iterator for Elaborator < I , O > { type Item = O ; fn size_hint (& self) -> (usize , Option < usize >) { (self . stack . len () , None) } fn next (& mut self) -> Option < Self :: Item > { if let Some (obligation) = self . stack . pop () { self . elaborate (& obligation) ; Some (obligation) } else { None } } }}}

macro_rules! supertrait_def_ids_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function supertrait_def_ids in module {}", module_path!());
    };
}

mkfn!{
    supertrait_def_ids_introspect!();
    # [doc = " Computes the def-ids of the transitive supertraits of `trait_def_id`. This (intentionally)"] # [doc = " does not compute the full elaborated super-predicates but just the set of def-ids. It is used"] # [doc = " to identify which traits may define a given associated type to help avoid cycle errors,"] # [doc = " and to make size estimates for vtable layout computation."] pub fn supertrait_def_ids < I : Interner > (cx : I , trait_def_id : I :: TraitId ,) -> impl Iterator < Item = I :: TraitId > { let mut set = HashSet :: default () ; let mut stack = vec ! [trait_def_id] ; set . insert (trait_def_id) ; std :: iter :: from_fn (move | | { let trait_def_id = stack . pop () ? ; for (predicate , _) in cx . explicit_super_predicates_of (trait_def_id) . iter_identity () { if let ty :: ClauseKind :: Trait (data) = predicate . kind () . skip_binder () && set . insert (data . def_id ()) { stack . push (data . def_id ()) ; } } Some (trait_def_id) }) }
}

macro_rules! supertraits_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function supertraits in module {}", module_path!());
    };
}

mkfn!{
    supertraits_introspect!();
    pub fn supertraits < I : Interner > (cx : I , trait_ref : ty :: Binder < I , ty :: TraitRef < I > > ,) -> FilterToTraits < I , Elaborator < I , I :: Clause > > { elaborate (cx , [trait_ref . upcast (cx)]) . filter_only_self () . filter_to_traits () }
}
mkitem!{mkimpl!{impl < I : Interner > Elaborator < I , I :: Clause > { fn filter_to_traits (self) -> FilterToTraits < I , Self > { FilterToTraits { _cx : PhantomData , base_iterator : self } } }}}
mkitem!{mkstruct!{# [doc = " A filter around an iterator of predicates that makes it yield up"] # [doc = " just trait references."] pub struct FilterToTraits < I : Interner , It : Iterator < Item = I :: Clause > > { _cx : PhantomData < I > , base_iterator : It , }}}
mkitem!{mkimpl!{impl < I : Interner , It : Iterator < Item = I :: Clause > > Iterator for FilterToTraits < I , It > { type Item = ty :: Binder < I , ty :: TraitRef < I > > ; fn next (& mut self) -> Option < ty :: Binder < I , ty :: TraitRef < I > > > { while let Some (pred) = self . base_iterator . next () { if let Some (data) = pred . as_trait_clause () { return Some (data . map_bound (| t | t . trait_ref)) ; } } None } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . base_iterator . size_hint () ; (0 , upper) } }}}

macro_rules! elaborate_outlives_assumptions_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function elaborate_outlives_assumptions in module {}", module_path!());
    };
}

mkfn!{
    elaborate_outlives_assumptions_introspect!();
    pub fn elaborate_outlives_assumptions < I : Interner > (cx : I , assumptions : impl IntoIterator < Item = ty :: OutlivesPredicate < I , I :: GenericArg > > ,) -> HashSet < ty :: OutlivesPredicate < I , I :: GenericArg > > { let mut collected = HashSet :: default () ; for ty :: OutlivesPredicate (arg1 , r2) in assumptions { collected . insert (ty :: OutlivesPredicate (arg1 , r2)) ; match arg1 . kind () { ty :: GenericArgKind :: Type (ty1) => { let mut components = smallvec ! [] ; push_outlives_components (cx , ty1 , & mut components) ; for c in components { match c { Component :: Region (r1) => { if ! r1 . is_bound () { collected . insert (ty :: OutlivesPredicate (r1 . into () , r2)) ; } } Component :: Param (p) => { let ty = Ty :: new_param (cx , p) ; collected . insert (ty :: OutlivesPredicate (ty . into () , r2)) ; } Component :: Placeholder (p) => { let ty = Ty :: new_placeholder (cx , p) ; collected . insert (ty :: OutlivesPredicate (ty . into () , r2)) ; } Component :: Alias (alias_ty) => { collected . insert (ty :: OutlivesPredicate (alias_ty . to_ty (cx) . into () , r2)) ; } Component :: UnresolvedInferenceVariable (_) | Component :: EscapingAlias (_) => { } } } } ty :: GenericArgKind :: Lifetime (_) => { } ty :: GenericArgKind :: Const (_) => { } } } collected }
}