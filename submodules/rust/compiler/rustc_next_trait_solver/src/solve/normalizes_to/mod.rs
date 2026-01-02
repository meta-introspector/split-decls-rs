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
mkmod!{anon_const, { 
                getname!(anon_const);
                getsrc!(anon_const);
                getpath!(anon_const);
                get_deps!(anon_const);
                get_crates!(anon_const);
                mkinclude!(anon_const);
                 
            }}
mkmod!{free_alias, { 
                getname!(free_alias);
                getsrc!(free_alias);
                getpath!(free_alias);
                get_deps!(free_alias);
                get_crates!(free_alias);
                mkinclude!(free_alias);
                 
            }}
mkmod!{inherent, { 
                getname!(inherent);
                getsrc!(inherent);
                getpath!(inherent);
                get_deps!(inherent);
                get_crates!(inherent);
                mkinclude!(inherent);
                 
            }}
mkmod!{opaque_types, { 
                getname!(opaque_types);
                getsrc!(opaque_types);
                getpath!(opaque_types);
                get_deps!(opaque_types);
                get_crates!(opaque_types);
                mkinclude!(opaque_types);
                 
            }}
mkuse!{use rustc_type_ir :: fast_reject :: DeepRejectCtxt ;}
mkuse!{use rustc_type_ir :: inherent :: * ;}
mkuse!{use rustc_type_ir :: lang_items :: { SolverAdtLangItem , SolverLangItem , SolverTraitLangItem } ;}
mkuse!{use rustc_type_ir :: solve :: SizedTraitKind ;}
mkuse!{use rustc_type_ir :: { self as ty , Interner , NormalizesTo , PredicateKind , Upcast as _ } ;}
mkuse!{use tracing :: instrument ;}
mkuse!{use crate :: delegate :: SolverDelegate ;}
mkuse!{use crate :: solve :: assembly :: structural_traits :: { self , AsyncCallableRelevantTypes } ;}
mkuse!{use crate :: solve :: assembly :: { self , Candidate } ;}
mkuse!{use crate :: solve :: inspect :: ProbeKind ;}
mkuse!{use crate :: solve :: { BuiltinImplSource , CandidateSource , Certainty , EvalCtxt , Goal , GoalSource , MaybeCause , NoSolution , QueryResult , } ;}
mkitem!{mkimpl!{impl < D , I > EvalCtxt < '_ , D > where D : SolverDelegate < Interner = I > , I : Interner , { # [instrument (level = "trace" , skip (self) , ret)] pub (super) fn compute_normalizes_to_goal (& mut self , goal : Goal < I , NormalizesTo < I > > ,) -> QueryResult < I > { debug_assert ! (self . term_is_fully_unconstrained (goal)) ; let cx = self . cx () ; match goal . predicate . alias . kind (cx) { ty :: AliasTermKind :: ProjectionTy | ty :: AliasTermKind :: ProjectionConst => { let trait_ref = goal . predicate . alias . trait_ref (cx) ; let (_ , proven_via) = self . probe (| _ | ProbeKind :: ShadowedEnvProbing) . enter (| ecx | { let trait_goal : Goal < I , ty :: TraitPredicate < I > > = goal . with (cx , trait_ref) ; ecx . compute_trait_goal (trait_goal) }) ? ; self . assemble_and_merge_candidates (proven_via , goal , | ecx | { ecx . probe (| & result | ProbeKind :: RigidAlias { result }) . enter (| this | { this . structurally_instantiate_normalizes_to_term (goal , goal . predicate . alias ,) ; this . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) }) }) } ty :: AliasTermKind :: InherentTy | ty :: AliasTermKind :: InherentConst => { self . normalize_inherent_associated_term (goal) } ty :: AliasTermKind :: OpaqueTy => self . normalize_opaque_type (goal) , ty :: AliasTermKind :: FreeTy | ty :: AliasTermKind :: FreeConst => { self . normalize_free_alias (goal) } ty :: AliasTermKind :: UnevaluatedConst => self . normalize_anon_const (goal) , } } # [doc = " When normalizing an associated item, constrain the expected term to `term`."] # [doc = ""] # [doc = " We know `term` to always be a fully unconstrained inference variable, so"] # [doc = " `eq` should never fail here. However, in case `term` contains aliases, we"] # [doc = " emit nested `AliasRelate` goals to structurally normalize the alias."] pub fn instantiate_normalizes_to_term (& mut self , goal : Goal < I , NormalizesTo < I > > , term : I :: Term ,) { self . eq (goal . param_env , goal . predicate . term , term) . expect ("expected goal term to be fully unconstrained") ; } # [doc = " Unlike `instantiate_normalizes_to_term` this instantiates the expected term"] # [doc = " with a rigid alias. Using this is pretty much always wrong."] pub fn structurally_instantiate_normalizes_to_term (& mut self , goal : Goal < I , NormalizesTo < I > > , term : ty :: AliasTerm < I > ,) { self . relate_rigid_alias_non_alias (goal . param_env , term , ty :: Invariant , goal . predicate . term) . expect ("expected goal term to be fully unconstrained") ; } }}}
mkitem!{mkimpl!{impl < D , I > assembly :: GoalKind < D > for NormalizesTo < I > where D : SolverDelegate < Interner = I > , I : Interner , { fn self_ty (self) -> I :: Ty { self . self_ty () } fn trait_ref (self , cx : I) -> ty :: TraitRef < I > { self . alias . trait_ref (cx) } fn with_replaced_self_ty (self , cx : I , self_ty : I :: Ty) -> Self { self . with_replaced_self_ty (cx , self_ty) } fn trait_def_id (self , cx : I) -> I :: TraitId { self . trait_def_id (cx) } fn fast_reject_assumption (ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > , assumption : I :: Clause ,) -> Result < () , NoSolution > { if let Some (projection_pred) = assumption . as_projection_clause () && projection_pred . item_def_id () == goal . predicate . def_id () && DeepRejectCtxt :: relate_rigid_rigid (ecx . cx ()) . args_may_unify (goal . predicate . alias . args , projection_pred . skip_binder () . projection_term . args ,) { Ok (()) } else { Err (NoSolution) } } fn match_assumption (ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > , assumption : I :: Clause , then : impl FnOnce (& mut EvalCtxt < '_ , D >) -> QueryResult < I > ,) -> QueryResult < I > { let cx = ecx . cx () ; match goal . predicate . alias . kind (cx) { ty :: AliasTermKind :: ProjectionTy | ty :: AliasTermKind :: ProjectionConst => { for arg in goal . predicate . alias . own_args (cx) . iter () { let Some (term) = arg . as_term () else { continue ; } ; let term = ecx . structurally_normalize_term (goal . param_env , term) ? ; if term . is_infer () { return ecx . evaluate_added_goals_and_make_canonical_response (Certainty :: AMBIGUOUS ,) ; } } } ty :: AliasTermKind :: OpaqueTy | ty :: AliasTermKind :: InherentTy | ty :: AliasTermKind :: InherentConst | ty :: AliasTermKind :: FreeTy | ty :: AliasTermKind :: FreeConst | ty :: AliasTermKind :: UnevaluatedConst => { } } let projection_pred = assumption . as_projection_clause () . unwrap () ; let assumption_projection_pred = ecx . instantiate_binder_with_infer (projection_pred) ; ecx . eq (goal . param_env , goal . predicate . alias , assumption_projection_pred . projection_term) ? ; ecx . instantiate_normalizes_to_term (goal , assumption_projection_pred . term) ; ecx . add_goals (GoalSource :: AliasWellFormed , cx . own_predicates_of (goal . predicate . def_id ()) . iter_instantiated (cx , goal . predicate . alias . args) . map (| pred | goal . with (cx , pred)) ,) ; then (ecx) } fn consider_additional_alias_assumptions (_ecx : & mut EvalCtxt < '_ , D > , _goal : Goal < I , Self > , _alias_ty : ty :: AliasTy < I > ,) -> Vec < Candidate < I > > { vec ! [] } fn consider_impl_candidate (ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , NormalizesTo < I > > , impl_def_id : I :: ImplId , then : impl FnOnce (& mut EvalCtxt < '_ , D > , Certainty) -> QueryResult < I > ,) -> Result < Candidate < I > , NoSolution > { let cx = ecx . cx () ; let goal_trait_ref = goal . predicate . alias . trait_ref (cx) ; let impl_trait_ref = cx . impl_trait_ref (impl_def_id) ; if ! DeepRejectCtxt :: relate_rigid_infer (ecx . cx ()) . args_may_unify (goal . predicate . alias . trait_ref (cx) . args , impl_trait_ref . skip_binder () . args ,) { return Err (NoSolution) ; } let impl_polarity = cx . impl_polarity (impl_def_id) ; match impl_polarity { ty :: ImplPolarity :: Negative => return Err (NoSolution) , ty :: ImplPolarity :: Reservation => { unimplemented ! ("reservation impl for trait with assoc item: {:?}" , goal) } ty :: ImplPolarity :: Positive => { } } ; ecx . probe_trait_candidate (CandidateSource :: Impl (impl_def_id)) . enter (| ecx | { let impl_args = ecx . fresh_args_for_item (impl_def_id . into ()) ; let impl_trait_ref = impl_trait_ref . instantiate (cx , impl_args) ; ecx . eq (goal . param_env , goal_trait_ref , impl_trait_ref) ? ; let where_clause_bounds = cx . predicates_of (impl_def_id . into ()) . iter_instantiated (cx , impl_args) . map (| pred | goal . with (cx , pred)) ; ecx . add_goals (GoalSource :: ImplWhereBound , where_clause_bounds) ; ecx . try_evaluate_added_goals () ? ; ecx . add_goals (GoalSource :: AliasWellFormed , cx . own_predicates_of (goal . predicate . def_id ()) . iter_instantiated (cx , goal . predicate . alias . args) . map (| pred | goal . with (cx , pred)) ,) ; let error_response = | ecx : & mut EvalCtxt < '_ , D > , guar | { let error_term = match goal . predicate . alias . kind (cx) { ty :: AliasTermKind :: ProjectionTy => Ty :: new_error (cx , guar) . into () , ty :: AliasTermKind :: ProjectionConst => Const :: new_error (cx , guar) . into () , kind => panic ! ("expected projection, found {kind:?}") , } ; ecx . instantiate_normalizes_to_term (goal , error_term) ; ecx . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) } ; let target_item_def_id = match ecx . fetch_eligible_assoc_item (goal_trait_ref , goal . predicate . def_id () , impl_def_id ,) { Ok (Some (target_item_def_id)) => target_item_def_id , Ok (None) => { match ecx . typing_mode () { ty :: TypingMode :: Coherence => { ecx . add_goal (GoalSource :: Misc , goal . with (cx , PredicateKind :: Ambiguous)) ; return ecx . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) ; } ty :: TypingMode :: Analysis { .. } | ty :: TypingMode :: Borrowck { .. } | ty :: TypingMode :: PostBorrowckAnalysis { .. } | ty :: TypingMode :: PostAnalysis => { ecx . structurally_instantiate_normalizes_to_term (goal , goal . predicate . alias ,) ; return ecx . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) ; } } ; } Err (guar) => return error_response (ecx , guar) , } ; if ! cx . has_item_definition (target_item_def_id) { if cx . impl_self_is_guaranteed_unsized (impl_def_id) { match ecx . typing_mode () { ty :: TypingMode :: Coherence => { ecx . add_goal (GoalSource :: Misc , goal . with (cx , PredicateKind :: Ambiguous)) ; return then (ecx , Certainty :: Yes) ; } ty :: TypingMode :: Analysis { .. } | ty :: TypingMode :: Borrowck { .. } | ty :: TypingMode :: PostBorrowckAnalysis { .. } | ty :: TypingMode :: PostAnalysis => { ecx . structurally_instantiate_normalizes_to_term (goal , goal . predicate . alias ,) ; return then (ecx , Certainty :: Yes) ; } } } else { return error_response (ecx , cx . delay_bug ("missing item")) ; } } let target_container_def_id = cx . parent (target_item_def_id) ; let target_args = ecx . translate_args (goal , impl_def_id , impl_args , impl_trait_ref , target_container_def_id ,) ? ; if ! cx . check_args_compatible (target_item_def_id , target_args) { return error_response (ecx , cx . delay_bug ("associated item has mismatched arguments") ,) ; } let term = match goal . predicate . alias . kind (cx) { ty :: AliasTermKind :: ProjectionTy => { cx . type_of (target_item_def_id) . map_bound (| ty | ty . into ()) } ty :: AliasTermKind :: ProjectionConst => { if cx . features () . associated_const_equality () { panic ! ("associated const projection is not supported yet") } else { ty :: EarlyBinder :: bind (Const :: new_error_with_message (cx , "associated const projection is not supported yet" ,) . into () ,) } } kind => panic ! ("expected projection, found {kind:?}") , } ; ecx . instantiate_normalizes_to_term (goal , term . instantiate (cx , target_args)) ; ecx . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) }) } # [doc = " Fail to normalize if the predicate contains an error, alternatively, we could normalize to `ty::Error`"] # [doc = " and succeed. Can experiment with this to figure out what results in better error messages."] fn consider_error_guaranteed_candidate (_ecx : & mut EvalCtxt < '_ , D > , _guar : I :: ErrorGuaranteed ,) -> Result < Candidate < I > , NoSolution > { Err (NoSolution) } fn consider_auto_trait_candidate (ecx : & mut EvalCtxt < '_ , D > , _goal : Goal < I , Self > ,) -> Result < Candidate < I > , NoSolution > { ecx . cx () . delay_bug ("associated types not allowed on auto traits") ; Err (NoSolution) } fn consider_trait_alias_candidate (_ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > ,) -> Result < Candidate < I > , NoSolution > { panic ! ("trait aliases do not have associated types: {:?}" , goal) ; } fn consider_builtin_sizedness_candidates (_ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > , _sizedness : SizedTraitKind ,) -> Result < Candidate < I > , NoSolution > { panic ! ("`Sized`/`MetaSized` does not have an associated type: {:?}" , goal) ; } fn consider_builtin_copy_clone_candidate (_ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > ,) -> Result < Candidate < I > , NoSolution > { panic ! ("`Copy`/`Clone` does not have an associated type: {:?}" , goal) ; } fn consider_builtin_fn_ptr_trait_candidate (_ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > ,) -> Result < Candidate < I > , NoSolution > { panic ! ("`FnPtr` does not have an associated type: {:?}" , goal) ; } fn consider_builtin_fn_trait_candidates (ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > , goal_kind : ty :: ClosureKind ,) -> Result < Candidate < I > , NoSolution > { let cx = ecx . cx () ; let tupled_inputs_and_output = match structural_traits :: extract_tupled_inputs_and_output_from_callable (cx , goal . predicate . self_ty () , goal_kind ,) ? { Some (tupled_inputs_and_output) => tupled_inputs_and_output , None => { return ecx . forced_ambiguity (MaybeCause :: Ambiguity) ; } } ; let output_is_sized_pred = tupled_inputs_and_output . map_bound (| (_ , output) | { ty :: TraitRef :: new (cx , cx . require_trait_lang_item (SolverTraitLangItem :: Sized) , [output]) }) ; let pred = tupled_inputs_and_output . map_bound (| (inputs , output) | ty :: ProjectionPredicate { projection_term : ty :: AliasTerm :: new (cx , goal . predicate . def_id () , [goal . predicate . self_ty () , inputs] ,) , term : output . into () , }) . upcast (cx) ; Self :: probe_and_consider_implied_clause (ecx , CandidateSource :: BuiltinImpl (BuiltinImplSource :: Misc) , goal , pred , [(GoalSource :: ImplWhereBound , goal . with (cx , output_is_sized_pred))] ,) } fn consider_builtin_async_fn_trait_candidates (ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > , goal_kind : ty :: ClosureKind ,) -> Result < Candidate < I > , NoSolution > { let cx = ecx . cx () ; let env_region = match goal_kind { ty :: ClosureKind :: Fn | ty :: ClosureKind :: FnMut => goal . predicate . alias . args . region_at (2) , ty :: ClosureKind :: FnOnce => Region :: new_static (cx) , } ; let (tupled_inputs_and_output_and_coroutine , nested_preds) = structural_traits :: extract_tupled_inputs_and_output_from_async_callable (cx , goal . predicate . self_ty () , goal_kind , env_region ,) ? ; let output_is_sized_pred = tupled_inputs_and_output_and_coroutine . map_bound (| AsyncCallableRelevantTypes { output_coroutine_ty : output_ty , .. } | { ty :: TraitRef :: new (cx , cx . require_trait_lang_item (SolverTraitLangItem :: Sized) , [output_ty] ,) } ,) ; let pred = tupled_inputs_and_output_and_coroutine . map_bound (| AsyncCallableRelevantTypes { tupled_inputs_ty , output_coroutine_ty , coroutine_return_ty , } | { let (projection_term , term) = if cx . is_lang_item (goal . predicate . def_id () , SolverLangItem :: CallOnceFuture) { (ty :: AliasTerm :: new (cx , goal . predicate . def_id () , [goal . predicate . self_ty () , tupled_inputs_ty] ,) , output_coroutine_ty . into () ,) } else if cx . is_lang_item (goal . predicate . def_id () , SolverLangItem :: CallRefFuture) { (ty :: AliasTerm :: new (cx , goal . predicate . def_id () , [I :: GenericArg :: from (goal . predicate . self_ty ()) , tupled_inputs_ty . into () , env_region . into () ,] ,) , output_coroutine_ty . into () ,) } else if cx . is_lang_item (goal . predicate . def_id () , SolverLangItem :: AsyncFnOnceOutput) { (ty :: AliasTerm :: new (cx , goal . predicate . def_id () , [I :: GenericArg :: from (goal . predicate . self_ty ()) , tupled_inputs_ty . into () ,] ,) , coroutine_return_ty . into () ,) } else { panic ! ("no such associated type in `AsyncFn*`: {:?}" , goal . predicate . def_id ()) } ; ty :: ProjectionPredicate { projection_term , term } } ,) . upcast (cx) ; Self :: probe_and_consider_implied_clause (ecx , CandidateSource :: BuiltinImpl (BuiltinImplSource :: Misc) , goal , pred , [goal . with (cx , output_is_sized_pred)] . into_iter () . chain (nested_preds . into_iter () . map (| pred | goal . with (cx , pred))) . map (| goal | (GoalSource :: ImplWhereBound , goal)) ,) } fn consider_builtin_async_fn_kind_helper_candidate (ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > ,) -> Result < Candidate < I > , NoSolution > { let [closure_fn_kind_ty , goal_kind_ty , borrow_region , tupled_inputs_ty , tupled_upvars_ty , coroutine_captures_by_ref_ty ,] = * goal . predicate . alias . args . as_slice () else { panic ! () ; } ; if tupled_upvars_ty . expect_ty () . is_ty_var () { return ecx . forced_ambiguity (MaybeCause :: Ambiguity) ; } let Some (closure_kind) = closure_fn_kind_ty . expect_ty () . to_opt_closure_kind () else { return Err (NoSolution) ; } ; let Some (goal_kind) = goal_kind_ty . expect_ty () . to_opt_closure_kind () else { return Err (NoSolution) ; } ; if ! closure_kind . extends (goal_kind) { return Err (NoSolution) ; } let upvars_ty = ty :: CoroutineClosureSignature :: tupled_upvars_by_closure_kind (ecx . cx () , goal_kind , tupled_inputs_ty . expect_ty () , tupled_upvars_ty . expect_ty () , coroutine_captures_by_ref_ty . expect_ty () , borrow_region . expect_region () ,) ; ecx . probe_builtin_trait_candidate (BuiltinImplSource :: Misc) . enter (| ecx | { ecx . instantiate_normalizes_to_term (goal , upvars_ty . into ()) ; ecx . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) }) } fn consider_builtin_tuple_candidate (_ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > ,) -> Result < Candidate < I > , NoSolution > { panic ! ("`Tuple` does not have an associated type: {:?}" , goal) ; } fn consider_builtin_pointee_candidate (ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > ,) -> Result < Candidate < I > , NoSolution > { let cx = ecx . cx () ; let metadata_def_id = cx . require_lang_item (SolverLangItem :: Metadata) ; assert_eq ! (metadata_def_id , goal . predicate . def_id ()) ; let metadata_ty = match goal . predicate . self_ty () . kind () { ty :: Bool | ty :: Char | ty :: Int (..) | ty :: Uint (..) | ty :: Float (..) | ty :: Array (..) | ty :: Pat (..) | ty :: RawPtr (..) | ty :: Ref (..) | ty :: FnDef (..) | ty :: FnPtr (..) | ty :: Closure (..) | ty :: CoroutineClosure (..) | ty :: Infer (ty :: IntVar (..) | ty :: FloatVar (..)) | ty :: Coroutine (..) | ty :: CoroutineWitness (..) | ty :: Never | ty :: Foreign (..) => Ty :: new_unit (cx) , ty :: Error (e) => Ty :: new_error (cx , e) , ty :: Str | ty :: Slice (_) => Ty :: new_usize (cx) , ty :: Dynamic (_ , _ , ty :: Dyn) => { let dyn_metadata = cx . require_lang_item (SolverLangItem :: DynMetadata) ; cx . type_of (dyn_metadata) . instantiate (cx , & [I :: GenericArg :: from (goal . predicate . self_ty ())]) } ty :: Alias (_ , _) | ty :: Param (_) | ty :: Placeholder (..) => { let alias_bound_result = ecx . probe_builtin_trait_candidate (BuiltinImplSource :: Misc) . enter (| ecx | { let sized_predicate = ty :: TraitRef :: new (cx , cx . require_trait_lang_item (SolverTraitLangItem :: Sized) , [I :: GenericArg :: from (goal . predicate . self_ty ())] ,) ; ecx . add_goal (GoalSource :: Misc , goal . with (cx , sized_predicate)) ; ecx . instantiate_normalizes_to_term (goal , Ty :: new_unit (cx) . into ()) ; ecx . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) }) ; return alias_bound_result . or_else (| NoSolution | { ecx . probe_builtin_trait_candidate (BuiltinImplSource :: Misc) . enter (| this | { this . structurally_instantiate_normalizes_to_term (goal , goal . predicate . alias ,) ; this . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) }) }) ; } ty :: Adt (def , args) if def . is_struct () => match def . struct_tail_ty (cx) { None => Ty :: new_unit (cx) , Some (tail_ty) => { Ty :: new_projection (cx , metadata_def_id , [tail_ty . instantiate (cx , args)]) } } , ty :: Adt (_ , _) => Ty :: new_unit (cx) , ty :: Tuple (elements) => match elements . last () { None => Ty :: new_unit (cx) , Some (tail_ty) => Ty :: new_projection (cx , metadata_def_id , [tail_ty]) , } , ty :: UnsafeBinder (_) => { todo ! () } ty :: Infer (ty :: TyVar (_) | ty :: FreshTy (_) | ty :: FreshIntTy (_) | ty :: FreshFloatTy (_)) | ty :: Bound (..) => panic ! ("unexpected self ty `{:?}` when normalizing `<T as Pointee>::Metadata`" , goal . predicate . self_ty ()) , } ; ecx . probe_builtin_trait_candidate (BuiltinImplSource :: Misc) . enter (| ecx | { ecx . instantiate_normalizes_to_term (goal , metadata_ty . into ()) ; ecx . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) }) } fn consider_builtin_future_candidate (ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > ,) -> Result < Candidate < I > , NoSolution > { let self_ty = goal . predicate . self_ty () ; let ty :: Coroutine (def_id , args) = self_ty . kind () else { return Err (NoSolution) ; } ; let cx = ecx . cx () ; if ! cx . coroutine_is_async (def_id) { return Err (NoSolution) ; } let term = args . as_coroutine () . return_ty () . into () ; Self :: probe_and_consider_implied_clause (ecx , CandidateSource :: BuiltinImpl (BuiltinImplSource :: Misc) , goal , ty :: ProjectionPredicate { projection_term : ty :: AliasTerm :: new (ecx . cx () , goal . predicate . def_id () , [self_ty]) , term , } . upcast (cx) , [] ,) } fn consider_builtin_iterator_candidate (ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > ,) -> Result < Candidate < I > , NoSolution > { let self_ty = goal . predicate . self_ty () ; let ty :: Coroutine (def_id , args) = self_ty . kind () else { return Err (NoSolution) ; } ; let cx = ecx . cx () ; if ! cx . coroutine_is_gen (def_id) { return Err (NoSolution) ; } let term = args . as_coroutine () . yield_ty () . into () ; Self :: probe_and_consider_implied_clause (ecx , CandidateSource :: BuiltinImpl (BuiltinImplSource :: Misc) , goal , ty :: ProjectionPredicate { projection_term : ty :: AliasTerm :: new (ecx . cx () , goal . predicate . def_id () , [self_ty]) , term , } . upcast (cx) , [] ,) } fn consider_builtin_fused_iterator_candidate (_ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > ,) -> Result < Candidate < I > , NoSolution > { panic ! ("`FusedIterator` does not have an associated type: {:?}" , goal) ; } fn consider_builtin_async_iterator_candidate (ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > ,) -> Result < Candidate < I > , NoSolution > { let self_ty = goal . predicate . self_ty () ; let ty :: Coroutine (def_id , args) = self_ty . kind () else { return Err (NoSolution) ; } ; let cx = ecx . cx () ; if ! cx . coroutine_is_async_gen (def_id) { return Err (NoSolution) ; } ecx . probe_builtin_trait_candidate (BuiltinImplSource :: Misc) . enter (| ecx | { let expected_ty = ecx . next_ty_infer () ; let wrapped_expected_ty = Ty :: new_adt (cx , cx . adt_def (cx . require_adt_lang_item (SolverAdtLangItem :: Poll)) , cx . mk_args (& [Ty :: new_adt (cx , cx . adt_def (cx . require_adt_lang_item (SolverAdtLangItem :: Option)) , cx . mk_args (& [expected_ty . into ()]) ,) . into ()]) ,) ; let yield_ty = args . as_coroutine () . yield_ty () ; ecx . eq (goal . param_env , wrapped_expected_ty , yield_ty) ? ; ecx . instantiate_normalizes_to_term (goal , expected_ty . into ()) ; ecx . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) }) } fn consider_builtin_coroutine_candidate (ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > ,) -> Result < Candidate < I > , NoSolution > { let self_ty = goal . predicate . self_ty () ; let ty :: Coroutine (def_id , args) = self_ty . kind () else { return Err (NoSolution) ; } ; let cx = ecx . cx () ; if ! cx . is_general_coroutine (def_id) { return Err (NoSolution) ; } let coroutine = args . as_coroutine () ; let term = if cx . is_lang_item (goal . predicate . def_id () , SolverLangItem :: CoroutineReturn) { coroutine . return_ty () . into () } else if cx . is_lang_item (goal . predicate . def_id () , SolverLangItem :: CoroutineYield) { coroutine . yield_ty () . into () } else { panic ! ("unexpected associated item `{:?}` for `{self_ty:?}`" , goal . predicate . def_id ()) } ; Self :: probe_and_consider_implied_clause (ecx , CandidateSource :: BuiltinImpl (BuiltinImplSource :: Misc) , goal , ty :: ProjectionPredicate { projection_term : ty :: AliasTerm :: new (ecx . cx () , goal . predicate . def_id () , [self_ty , coroutine . resume_ty ()] ,) , term , } . upcast (cx) , [] ,) } fn consider_structural_builtin_unsize_candidates (_ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > ,) -> Vec < Candidate < I > > { panic ! ("`Unsize` does not have an associated type: {:?}" , goal) ; } fn consider_builtin_discriminant_kind_candidate (ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > ,) -> Result < Candidate < I > , NoSolution > { let self_ty = goal . predicate . self_ty () ; let discriminant_ty = match self_ty . kind () { ty :: Bool | ty :: Char | ty :: Int (..) | ty :: Uint (..) | ty :: Float (..) | ty :: Array (..) | ty :: Pat (..) | ty :: RawPtr (..) | ty :: Ref (..) | ty :: FnDef (..) | ty :: FnPtr (..) | ty :: Closure (..) | ty :: CoroutineClosure (..) | ty :: Infer (ty :: IntVar (..) | ty :: FloatVar (..)) | ty :: Coroutine (..) | ty :: CoroutineWitness (..) | ty :: Never | ty :: Foreign (..) | ty :: Adt (_ , _) | ty :: Str | ty :: Slice (_) | ty :: Dynamic (_ , _ , _) | ty :: Tuple (_) | ty :: Error (_) => self_ty . discriminant_ty (ecx . cx ()) , ty :: UnsafeBinder (_) => { todo ! ("discr subgoal...") } ty :: Alias (_ , _) | ty :: Param (_) | ty :: Placeholder (..) => { return ecx . probe_builtin_trait_candidate (BuiltinImplSource :: Misc) . enter (| ecx | { ecx . structurally_instantiate_normalizes_to_term (goal , goal . predicate . alias) ; ecx . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) }) ; } ty :: Infer (ty :: TyVar (_) | ty :: FreshTy (_) | ty :: FreshIntTy (_) | ty :: FreshFloatTy (_)) | ty :: Bound (..) => panic ! ("unexpected self ty `{:?}` when normalizing `<T as DiscriminantKind>::Discriminant`" , goal . predicate . self_ty ()) , } ; ecx . probe_builtin_trait_candidate (BuiltinImplSource :: Misc) . enter (| ecx | { ecx . instantiate_normalizes_to_term (goal , discriminant_ty . into ()) ; ecx . evaluate_added_goals_and_make_canonical_response (Certainty :: Yes) }) } fn consider_builtin_destruct_candidate (_ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > ,) -> Result < Candidate < I > , NoSolution > { panic ! ("`Destruct` does not have an associated type: {:?}" , goal) ; } fn consider_builtin_transmute_candidate (_ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > ,) -> Result < Candidate < I > , NoSolution > { panic ! ("`TransmuteFrom` does not have an associated type: {:?}" , goal) } fn consider_builtin_bikeshed_guaranteed_no_drop_candidate (_ecx : & mut EvalCtxt < '_ , D > , goal : Goal < I , Self > ,) -> Result < Candidate < I > , NoSolution > { unreachable ! ("`BikeshedGuaranteedNoDrop` does not have an associated type: {:?}" , goal) } }}}
mkitem!{mkimpl!{impl < D , I > EvalCtxt < '_ , D > where D : SolverDelegate < Interner = I > , I : Interner , { fn translate_args (& mut self , goal : Goal < I , ty :: NormalizesTo < I > > , impl_def_id : I :: ImplId , impl_args : I :: GenericArgs , impl_trait_ref : rustc_type_ir :: TraitRef < I > , target_container_def_id : I :: DefId ,) -> Result < I :: GenericArgs , NoSolution > { let cx = self . cx () ; Ok (if target_container_def_id == impl_trait_ref . def_id . into () { goal . predicate . alias . args } else if target_container_def_id == impl_def_id . into () { goal . predicate . alias . args . rebase_onto (cx , impl_trait_ref . def_id . into () , impl_args) } else { let target_args = self . fresh_args_for_item (target_container_def_id) ; let target_trait_ref = cx . impl_trait_ref (target_container_def_id . try_into () . unwrap ()) . instantiate (cx , target_args) ; self . eq (goal . param_env , impl_trait_ref , target_trait_ref) ? ; self . add_goals (GoalSource :: Misc , cx . predicates_of (target_container_def_id) . iter_instantiated (cx , target_args) . map (| pred | goal . with (cx , pred)) ,) ; goal . predicate . alias . args . rebase_onto (cx , impl_trait_ref . def_id . into () , target_args) }) } }}}