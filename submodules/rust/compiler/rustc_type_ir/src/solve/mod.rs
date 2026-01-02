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
mkmod!{inspect, { 
                getname!(inspect);
                getsrc!(inspect);
                getpath!(inspect);
                get_deps!(inspect);
                get_crates!(inspect);
                mkinclude!(inspect);
                 
            }}
mkuse!{use std :: hash :: Hash ;}
mkuse!{use derive_where :: derive_where ;}
mkuse!{#[cfg (feature = "nightly")] use rustc_macros :: { Decodable_NoContext , Encodable_NoContext , HashStable_NoContext } ;}
mkuse!{use rustc_type_ir_macros :: { Lift_Generic , TypeFoldable_Generic , TypeVisitable_Generic } ;}
mkuse!{use crate :: lang_items :: SolverTraitLangItem ;}
mkuse!{use crate :: search_graph :: PathKind ;}
mkuse!{use crate :: { self as ty , Canonical , CanonicalVarValues , Interner , Upcast } ;}
mkitem!{pub type CanonicalInput < I , T = < I as Interner > :: Predicate > = ty :: CanonicalQueryInput < I , QueryInput < I , T > > ;}
mkitem!{pub type CanonicalResponse < I > = Canonical < I , Response < I > > ;}
mkitem!{#[doc = " The result of evaluating a canonical query."] #[doc = ""] #[doc = " FIXME: We use a different type than the existing canonical queries. This is because"] #[doc = " we need to add a `Certainty` for `overflow` and may want to restructure this code without"] #[doc = " having to worry about changes to currently used code. Once we've made progress on this"] #[doc = " solver, merge the two responses again."] pub type QueryResult < I > = Result < CanonicalResponse < I > , NoSolution > ;}
mkitem!{mkstruct!{#[derive (Copy , Clone , Debug , Hash , PartialEq , Eq)] #[cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] pub struct NoSolution ;}}
mkitem!{mkstruct!{#[doc = " A goal is a statement, i.e. `predicate`, we want to prove"] #[doc = " given some assumptions, i.e. `param_env`."] #[doc = ""] #[doc = " Most of the time the `param_env` contains the `where`-bounds of the function"] #[doc = " we're currently typechecking while the `predicate` is some trait bound."] #[derive_where (Clone , Hash , PartialEq , Debug ; I : Interner , P)] #[derive_where (Copy ; I : Interner , P : Copy)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] #[cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct Goal < I : Interner , P > { pub param_env : I :: ParamEnv , pub predicate : P , }}}
mkitem!{mkimpl!{impl < I : Interner , P : Eq > Eq for Goal < I , P > { }}}
mkitem!{mkimpl!{impl < I : Interner , P > Goal < I , P > { pub fn new (cx : I , param_env : I :: ParamEnv , predicate : impl Upcast < I , P >) -> Goal < I , P > { Goal { param_env , predicate : predicate . upcast (cx) } } #[doc = " Updates the goal to one with a different `predicate` but the same `param_env`."] pub fn with < Q > (self , cx : I , predicate : impl Upcast < I , Q >) -> Goal < I , Q > { Goal { param_env : self . param_env , predicate : predicate . upcast (cx) } } }}}
mkitem!{mkenum!{#[doc = " Why a specific goal has to be proven."] #[doc = ""] #[doc = " This is necessary as we treat nested goals different depending on"] #[doc = " their source. This is used to decide whether a cycle is coinductive."] #[doc = " See the documentation of `EvalCtxt::step_kind_for_source` for more details"] #[doc = " about this."] #[doc = ""] #[doc = " It is also used by proof tree visitors, e.g. for diagnostics purposes."] #[derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] #[cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] pub enum GoalSource { Misc , #[doc = " A nested goal required to prove that types are equal/subtypes."] #[doc = " This is always an unproductive step."] #[doc = ""] #[doc = " This is also used for all `NormalizesTo` goals as we they are used"] #[doc = " to relate types in `AliasRelate`."] TypeRelating , #[doc = " We're proving a where-bound of an impl."] ImplWhereBound , #[doc = " Const conditions that need to hold for `[const]` alias bounds to hold."] AliasBoundConstCondition , #[doc = " Instantiating a higher-ranked goal and re-proving it."] InstantiateHigherRanked , #[doc = " Predicate required for an alias projection to be well-formed."] #[doc = " This is used in three places:"] #[doc = " 1. projecting to an opaque whose hidden type is already registered in"] #[doc = "    the opaque type storage,"] #[doc = " 2. for rigid projections's trait goal,"] #[doc = " 3. for GAT where clauses."] AliasWellFormed , #[doc = " In case normalizing aliases in nested goals cycles, eagerly normalizing these"] #[doc = " aliases in the context of the parent may incorrectly change the cycle kind."] #[doc = " Normalizing aliases in goals therefore tracks the original path kind for this"] #[doc = " nested goal. See the comment of the `ReplaceAliasWithInfer` visitor for more"] #[doc = " details."] NormalizeGoal (PathKind) , }}}
mkitem!{mkstruct!{#[derive_where (Clone , Hash , PartialEq , Debug ; I : Interner , Goal < I , P >)] #[derive_where (Copy ; I : Interner , Goal < I , P >: Copy)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic)] #[cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct QueryInput < I : Interner , P > { pub goal : Goal < I , P > , pub predefined_opaques_in_body : I :: PredefinedOpaques , }}}
mkitem!{mkimpl!{impl < I : Interner , P : Eq > Eq for QueryInput < I , P > { }}}
mkitem!{mkstruct!{#[doc = " Opaques that are defined in the inference context before a query is called."] #[derive_where (Clone , Hash , PartialEq , Debug , Default ; I : Interner)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic)] #[cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct PredefinedOpaquesData < I : Interner > { pub opaque_types : Vec < (ty :: OpaqueTypeKey < I > , I :: Ty) > , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for PredefinedOpaquesData < I > { }}}
mkitem!{mkenum!{#[doc = " Possible ways the given goal can be proven."] #[derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] pub enum CandidateSource < I : Interner > { #[doc = " A user written impl."] #[doc = ""] #[doc = " ## Examples"] #[doc = ""] #[doc = " ```rust"] #[doc = " fn main() {"] #[doc = "     let x: Vec<u32> = Vec::new();"] #[doc = "     // This uses the impl from the standard library to prove `Vec<T>: Clone`."] #[doc = "     let y = x.clone();"] #[doc = " }"] #[doc = " ```"] Impl (I :: ImplId) , #[doc = " A builtin impl generated by the compiler. When adding a new special"] #[doc = " trait, try to use actual impls whenever possible. Builtin impls should"] #[doc = " only be used in cases where the impl cannot be manually be written."] #[doc = ""] #[doc = " Notable examples are auto traits, `Sized`, and `DiscriminantKind`."] #[doc = " For a list of all traits with builtin impls, check out the"] #[doc = " `EvalCtxt::assemble_builtin_impl_candidates` method."] BuiltinImpl (BuiltinImplSource) , #[doc = " An assumption from the environment. Stores a [`ParamEnvSource`], since we"] #[doc = " prefer non-global param-env candidates in candidate assembly."] #[doc = ""] #[doc = " ## Examples"] #[doc = ""] #[doc = " ```rust"] #[doc = " fn is_clone<T: Clone>(x: T) -> (T, T) {"] #[doc = "     // This uses the assumption `T: Clone` from the `where`-bounds"] #[doc = "     // to prove `T: Clone`."] #[doc = "     (x.clone(), x)"] #[doc = " }"] #[doc = " ```"] ParamEnv (ParamEnvSource) , #[doc = " If the self type is an alias type, e.g. an opaque type or a projection,"] #[doc = " we know the bounds on that alias to hold even without knowing its concrete"] #[doc = " underlying type."] #[doc = ""] #[doc = " More precisely this candidate is using the `n-th` bound in the `item_bounds` of"] #[doc = " the self type."] #[doc = ""] #[doc = " ## Examples"] #[doc = ""] #[doc = " ```rust"] #[doc = " trait Trait {"] #[doc = "     type Assoc: Clone;"] #[doc = " }"] #[doc = ""] #[doc = " fn foo<T: Trait>(x: <T as Trait>::Assoc) {"] #[doc = "     // We prove `<T as Trait>::Assoc` by looking at the bounds on `Assoc` in"] #[doc = "     // in the trait definition."] #[doc = "     let _y = x.clone();"] #[doc = " }"] #[doc = " ```"] AliasBound , #[doc = " A candidate that is registered only during coherence to represent some"] #[doc = " yet-unknown impl that could be produced downstream without violating orphan"] #[doc = " rules."] CoherenceUnknowable , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for CandidateSource < I > { }}}
mkitem!{mkenum!{#[derive (Clone , Copy , Hash , PartialEq , Eq , Debug)] pub enum ParamEnvSource { #[doc = " Preferred eagerly."] NonGlobal , Global , }}}
mkitem!{mkenum!{#[derive (Clone , Copy , Hash , PartialEq , Eq , Debug)] #[cfg_attr (feature = "nightly" , derive (HashStable_NoContext , Encodable_NoContext , Decodable_NoContext))] pub enum BuiltinImplSource { #[doc = " A built-in impl that is considered trivial, without any nested requirements. They"] #[doc = " are preferred over where-clauses, and we want to track them explicitly."] Trivial , #[doc = " Some built-in impl we don't need to differentiate. This should be used"] #[doc = " unless more specific information is necessary."] Misc , #[doc = " A built-in impl for trait objects. The index is only used in winnowing."] Object (usize) , #[doc = " A built-in implementation of `Upcast` for trait objects to other trait objects."] #[doc = ""] #[doc = " The index is only used for winnowing."] TraitUpcasting (usize) , }}}
mkitem!{mkstruct!{#[derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic)] #[cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] pub struct Response < I : Interner > { pub certainty : Certainty , pub var_values : CanonicalVarValues < I > , #[doc = " Additional constraints returned by this query."] pub external_constraints : I :: ExternalConstraints , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for Response < I > { }}}
mkitem!{mkstruct!{#[doc = " Additional constraints returned on success."] #[derive_where (Clone , Hash , PartialEq , Debug , Default ; I : Interner)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic)] #[cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] pub struct ExternalConstraintsData < I : Interner > { pub region_constraints : Vec < ty :: OutlivesPredicate < I , I :: GenericArg > > , pub opaque_types : Vec < (ty :: OpaqueTypeKey < I > , I :: Ty) > , pub normalization_nested_goals : NestedNormalizationGoals < I > , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for ExternalConstraintsData < I > { }}}
mkitem!{mkimpl!{impl < I : Interner > ExternalConstraintsData < I > { pub fn is_empty (& self) -> bool { self . region_constraints . is_empty () && self . opaque_types . is_empty () && self . normalization_nested_goals . is_empty () } }}}
mkitem!{mkstruct!{#[derive_where (Clone , Hash , PartialEq , Debug , Default ; I : Interner)] #[derive (TypeVisitable_Generic , TypeFoldable_Generic)] #[cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] pub struct NestedNormalizationGoals < I : Interner > (pub Vec < (GoalSource , Goal < I , I :: Predicate >) >) ;}}
mkitem!{mkimpl!{impl < I : Interner > Eq for NestedNormalizationGoals < I > { }}}
mkitem!{mkimpl!{impl < I : Interner > NestedNormalizationGoals < I > { pub fn empty () -> Self { NestedNormalizationGoals (vec ! []) } pub fn is_empty (& self) -> bool { self . 0 . is_empty () } }}}
mkitem!{mkenum!{#[derive (Clone , Copy , Hash , PartialEq , Eq , Debug)] #[cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] pub enum Certainty { Yes , Maybe (MaybeCause) , }}}
mkitem!{mkimpl!{impl Certainty { pub const AMBIGUOUS : Certainty = Certainty :: Maybe (MaybeCause :: Ambiguity) ; #[doc = " Use this function to merge the certainty of multiple nested subgoals."] #[doc = ""] #[doc = " Given an impl like `impl<T: Foo + Bar> Baz for T {}`, we have 2 nested"] #[doc = " subgoals whenever we use the impl as a candidate: `T: Foo` and `T: Bar`."] #[doc = " If evaluating `T: Foo` results in ambiguity and `T: Bar` results in"] #[doc = " success, we merge these two responses. This results in ambiguity."] #[doc = ""] #[doc = " If we unify ambiguity with overflow, we return overflow. This doesn't matter"] #[doc = " inside of the solver as we do not distinguish ambiguity from overflow. It does"] #[doc = " however matter for diagnostics. If `T: Foo` resulted in overflow and `T: Bar`"] #[doc = " in ambiguity without changing the inference state, we still want to tell the"] #[doc = " user that `T: Baz` results in overflow."] pub fn and (self , other : Certainty) -> Certainty { match (self , other) { (Certainty :: Yes , Certainty :: Yes) => Certainty :: Yes , (Certainty :: Yes , Certainty :: Maybe (_)) => other , (Certainty :: Maybe (_) , Certainty :: Yes) => self , (Certainty :: Maybe (a) , Certainty :: Maybe (b)) => Certainty :: Maybe (a . and (b)) , } } pub const fn overflow (suggest_increasing_limit : bool) -> Certainty { Certainty :: Maybe (MaybeCause :: Overflow { suggest_increasing_limit , keep_constraints : false }) } }}}
mkitem!{mkenum!{#[doc = " Why we failed to evaluate a goal."] #[derive (Clone , Copy , Hash , PartialEq , Eq , Debug)] #[cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] pub enum MaybeCause { #[doc = " We failed due to ambiguity. This ambiguity can either"] #[doc = " be a true ambiguity, i.e. there are multiple different answers,"] #[doc = " or we hit a case where we just don't bother, e.g. `?x: Trait` goals."] Ambiguity , #[doc = " We gave up due to an overflow, most often by hitting the recursion limit."] Overflow { suggest_increasing_limit : bool , keep_constraints : bool } , }}}
mkitem!{mkimpl!{impl MaybeCause { fn and (self , other : MaybeCause) -> MaybeCause { match (self , other) { (MaybeCause :: Ambiguity , MaybeCause :: Ambiguity) => MaybeCause :: Ambiguity , (MaybeCause :: Ambiguity , MaybeCause :: Overflow { .. }) => other , (MaybeCause :: Overflow { .. } , MaybeCause :: Ambiguity) => self , (MaybeCause :: Overflow { suggest_increasing_limit : limit_a , keep_constraints : keep_a , } , MaybeCause :: Overflow { suggest_increasing_limit : limit_b , keep_constraints : keep_b , } ,) => MaybeCause :: Overflow { suggest_increasing_limit : limit_a && limit_b , keep_constraints : keep_a && keep_b , } , } } pub fn or (self , other : MaybeCause) -> MaybeCause { match (self , other) { (MaybeCause :: Ambiguity , MaybeCause :: Ambiguity) => MaybeCause :: Ambiguity , (MaybeCause :: Ambiguity , MaybeCause :: Overflow { suggest_increasing_limit , keep_constraints : _ } ,) => MaybeCause :: Overflow { suggest_increasing_limit , keep_constraints : true } , (MaybeCause :: Overflow { suggest_increasing_limit , keep_constraints : _ } , MaybeCause :: Ambiguity ,) => MaybeCause :: Overflow { suggest_increasing_limit , keep_constraints : true } , (MaybeCause :: Overflow { suggest_increasing_limit : limit_a , keep_constraints : keep_a , } , MaybeCause :: Overflow { suggest_increasing_limit : limit_b , keep_constraints : keep_b , } ,) => MaybeCause :: Overflow { suggest_increasing_limit : limit_a || limit_b , keep_constraints : keep_a || keep_b , } , } } }}}
mkitem!{mkenum!{#[doc = " Indicates that a `impl Drop for Adt` is `const` or not."] #[derive (Debug)] pub enum AdtDestructorKind { NotConst , Const , }}}
mkitem!{mkenum!{#[doc = " Which sizedness trait - `Sized`, `MetaSized`? `PointeeSized` is omitted as it is removed during"] #[doc = " lowering."] #[derive (Copy , Clone , Debug , Eq , Hash , PartialEq)] #[cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] pub enum SizedTraitKind { #[doc = " `Sized` trait"] Sized , #[doc = " `MetaSized` trait"] MetaSized , }}}
mkitem!{mkimpl!{impl SizedTraitKind { #[doc = " Returns `DefId` of corresponding language item."] pub fn require_lang_item < I : Interner > (self , cx : I) -> I :: TraitId { cx . require_trait_lang_item (match self { SizedTraitKind :: Sized => SolverTraitLangItem :: Sized , SizedTraitKind :: MetaSized => SolverTraitLangItem :: MetaSized , }) } }}}