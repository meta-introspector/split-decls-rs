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
mkuse!{use rustc_type_ir :: data_structures :: { HashMap , ensure_sufficient_stack } ;}
mkuse!{use rustc_type_ir :: inherent :: * ;}
mkuse!{use rustc_type_ir :: solve :: { Goal , QueryInput } ;}
mkuse!{use rustc_type_ir :: { self as ty , Canonical , CanonicalParamEnvCacheEntry , CanonicalVarKind , Flags , InferCtxtLike , Interner , TypeFlags , TypeFoldable , TypeFolder , TypeSuperFoldable , TypeVisitableExt , } ;}
mkuse!{use crate :: delegate :: SolverDelegate ;}
mkitem!{# [doc = " Does this have infer/placeholder/param, free regions or ReErased?"] const NEEDS_CANONICAL : TypeFlags = TypeFlags :: from_bits (TypeFlags :: HAS_INFER . bits () | TypeFlags :: HAS_PLACEHOLDER . bits () | TypeFlags :: HAS_PARAM . bits () | TypeFlags :: HAS_FREE_REGIONS . bits () | TypeFlags :: HAS_RE_ERASED . bits () ,) . unwrap () ;}
mkitem!{mkenum!{# [derive (Debug , Clone , Copy)] enum CanonicalizeInputKind { # [doc = " When canonicalizing the `param_env`, we keep `'static` as merging"] # [doc = " trait candidates relies on it when deciding whether a where-bound"] # [doc = " is trivial."] ParamEnv , # [doc = " When canonicalizing predicates, we don't keep `'static`."] Predicate , }}}
mkitem!{mkenum!{# [doc = " Whether we're canonicalizing a query input or the query response."] # [doc = ""] # [doc = " When canonicalizing an input we're in the context of the caller"] # [doc = " while canonicalizing the response happens in the context of the"] # [doc = " query."] # [derive (Debug , Clone , Copy)] enum CanonicalizeMode { Input (CanonicalizeInputKind) , # [doc = " FIXME: We currently return region constraints referring to"] # [doc = " placeholders and inference variables from a binder instantiated"] # [doc = " inside of the query."] # [doc = ""] # [doc = " In the long term we should eagerly deal with these constraints"] # [doc = " inside of the query and only propagate constraints which are"] # [doc = " actually nameable by the caller."] Response { # [doc = " The highest universe nameable by the caller."] # [doc = ""] # [doc = " All variables in a universe nameable by the caller get mapped"] # [doc = " to the root universe in the response and then mapped back to"] # [doc = " their correct universe when applying the query response in the"] # [doc = " context of the caller."] # [doc = ""] # [doc = " This doesn't work for universes created inside of the query so"] # [doc = " we do remember their universe in the response."] max_input_universe : ty :: UniverseIndex , } , }}}
mkitem!{mkstruct!{pub struct Canonicalizer < 'a , D : SolverDelegate < Interner = I > , I : Interner > { delegate : & 'a D , canonicalize_mode : CanonicalizeMode , variables : & 'a mut Vec < I :: GenericArg > , var_kinds : Vec < CanonicalVarKind < I > > , variable_lookup_table : HashMap < I :: GenericArg , usize > , # [doc = " Maps each `sub_unification_table_root_var` to the index of the first"] # [doc = " variable which used it."] # [doc = ""] # [doc = " This means in case two type variables have the same sub relations root,"] # [doc = " we set the `sub_root` of the second variable to the position of the first."] # [doc = " Otherwise the `sub_root` of each type variable is just its own position."] sub_root_lookup_table : HashMap < ty :: TyVid , usize > , binder_index : ty :: DebruijnIndex , # [doc = " We only use the debruijn index during lookup. We don't need to"] # [doc = " track the `variables` as each generic arg only results in a single"] # [doc = " bound variable regardless of how many times it is encountered."] cache : HashMap < (ty :: DebruijnIndex , I :: Ty) , I :: Ty > , }}}
mkitem!{mkimpl!{impl < 'a , D : SolverDelegate < Interner = I > , I : Interner > Canonicalizer < 'a , D , I > { pub fn canonicalize_response < T : TypeFoldable < I > > (delegate : & 'a D , max_input_universe : ty :: UniverseIndex , variables : & 'a mut Vec < I :: GenericArg > , value : T ,) -> ty :: Canonical < I , T > { let mut canonicalizer = Canonicalizer { delegate , canonicalize_mode : CanonicalizeMode :: Response { max_input_universe } , variables , variable_lookup_table : Default :: default () , sub_root_lookup_table : Default :: default () , var_kinds : Vec :: new () , binder_index : ty :: INNERMOST , cache : Default :: default () , } ; let value = if value . has_type_flags (NEEDS_CANONICAL) { value . fold_with (& mut canonicalizer) } else { value } ; debug_assert ! (! value . has_infer () , "unexpected infer in {value:?}") ; debug_assert ! (! value . has_placeholders () , "unexpected placeholders in {value:?}") ; let (max_universe , variables) = canonicalizer . finalize () ; Canonical { max_universe , variables , value } } fn canonicalize_param_env (delegate : & 'a D , variables : & 'a mut Vec < I :: GenericArg > , param_env : I :: ParamEnv ,) -> (I :: ParamEnv , HashMap < I :: GenericArg , usize > , Vec < CanonicalVarKind < I > >) { if ! param_env . has_type_flags (NEEDS_CANONICAL) { return (param_env , Default :: default () , Vec :: new ()) ; } if ! param_env . has_non_region_infer () { delegate . cx () . canonical_param_env_cache_get_or_insert (param_env , | | { let mut variables = Vec :: new () ; let mut env_canonicalizer = Canonicalizer { delegate , canonicalize_mode : CanonicalizeMode :: Input (CanonicalizeInputKind :: ParamEnv) , variables : & mut variables , variable_lookup_table : Default :: default () , sub_root_lookup_table : Default :: default () , var_kinds : Vec :: new () , binder_index : ty :: INNERMOST , cache : Default :: default () , } ; let param_env = param_env . fold_with (& mut env_canonicalizer) ; debug_assert_eq ! (env_canonicalizer . binder_index , ty :: INNERMOST) ; debug_assert ! (env_canonicalizer . sub_root_lookup_table . is_empty ()) ; CanonicalParamEnvCacheEntry { param_env , variable_lookup_table : env_canonicalizer . variable_lookup_table , var_kinds : env_canonicalizer . var_kinds , variables , } } , | & CanonicalParamEnvCacheEntry { param_env , variables : ref cache_variables , ref variable_lookup_table , ref var_kinds , } | { debug_assert ! (variables . is_empty ()) ; variables . extend (cache_variables . iter () . copied ()) ; (param_env , variable_lookup_table . clone () , var_kinds . clone ()) } ,) } else { let mut env_canonicalizer = Canonicalizer { delegate , canonicalize_mode : CanonicalizeMode :: Input (CanonicalizeInputKind :: ParamEnv) , variables , variable_lookup_table : Default :: default () , sub_root_lookup_table : Default :: default () , var_kinds : Vec :: new () , binder_index : ty :: INNERMOST , cache : Default :: default () , } ; let param_env = param_env . fold_with (& mut env_canonicalizer) ; debug_assert_eq ! (env_canonicalizer . binder_index , ty :: INNERMOST) ; debug_assert ! (env_canonicalizer . sub_root_lookup_table . is_empty ()) ; (param_env , env_canonicalizer . variable_lookup_table , env_canonicalizer . var_kinds) } } # [doc = " When canonicalizing query inputs, we keep `'static` in the `param_env`"] # [doc = " but erase it everywhere else. We generally don't want to depend on region"] # [doc = " identity, so while it should not matter whether `'static` is kept in the"] # [doc = " value or opaque type storage as well, this prevents us from accidentally"] # [doc = " relying on it in the future."] # [doc = ""] # [doc = " We want to keep the option of canonicalizing `'static` to an existential"] # [doc = " variable in the future by changing the way we detect global where-bounds."] pub fn canonicalize_input < P : TypeFoldable < I > > (delegate : & 'a D , variables : & 'a mut Vec < I :: GenericArg > , input : QueryInput < I , P > ,) -> ty :: Canonical < I , QueryInput < I , P > > { let (param_env , variable_lookup_table , var_kinds) = Canonicalizer :: canonicalize_param_env (delegate , variables , input . goal . param_env) ; let mut rest_canonicalizer = Canonicalizer { delegate , canonicalize_mode : CanonicalizeMode :: Input (CanonicalizeInputKind :: Predicate) , variables , variable_lookup_table , sub_root_lookup_table : Default :: default () , var_kinds , binder_index : ty :: INNERMOST , cache : Default :: default () , } ; let predicate = input . goal . predicate ; let predicate = if predicate . has_type_flags (NEEDS_CANONICAL) { predicate . fold_with (& mut rest_canonicalizer) } else { predicate } ; let goal = Goal { param_env , predicate } ; let predefined_opaques_in_body = input . predefined_opaques_in_body ; let predefined_opaques_in_body = if input . predefined_opaques_in_body . has_type_flags (NEEDS_CANONICAL) { predefined_opaques_in_body . fold_with (& mut rest_canonicalizer) } else { predefined_opaques_in_body } ; let value = QueryInput { goal , predefined_opaques_in_body } ; debug_assert ! (! value . has_infer () , "unexpected infer in {value:?}") ; debug_assert ! (! value . has_placeholders () , "unexpected placeholders in {value:?}") ; let (max_universe , variables) = rest_canonicalizer . finalize () ; Canonical { max_universe , variables , value } } fn get_or_insert_bound_var (& mut self , arg : impl Into < I :: GenericArg > , kind : CanonicalVarKind < I > ,) -> ty :: BoundVar { let arg = arg . into () ; let idx = if self . variables . len () > 16 { if self . variable_lookup_table . is_empty () { self . variable_lookup_table . extend (self . variables . iter () . copied () . zip (0 ..)) ; } * self . variable_lookup_table . entry (arg) . or_insert_with (| | { let var = self . variables . len () ; self . variables . push (arg) ; self . var_kinds . push (kind) ; var }) } else { self . variables . iter () . position (| & v | v == arg) . unwrap_or_else (| | { let var = self . variables . len () ; self . variables . push (arg) ; self . var_kinds . push (kind) ; var }) } ; ty :: BoundVar :: from (idx) } fn get_or_insert_sub_root (& mut self , vid : ty :: TyVid) -> ty :: BoundVar { let root_vid = self . delegate . sub_unification_table_root_var (vid) ; let idx = * self . sub_root_lookup_table . entry (root_vid) . or_insert_with (| | self . variables . len ()) ; ty :: BoundVar :: from (idx) } fn finalize (self) -> (ty :: UniverseIndex , I :: CanonicalVarKinds) { let mut var_kinds = self . var_kinds ; match self . canonicalize_mode { CanonicalizeMode :: Input { .. } => { debug_assert ! (var_kinds . iter () . all (| var | var . universe () == ty :: UniverseIndex :: ROOT) , "expected all vars to be canonicalized in root universe: {var_kinds:#?}") ; let var_kinds = self . delegate . cx () . mk_canonical_var_kinds (& var_kinds) ; (ty :: UniverseIndex :: ROOT , var_kinds) } CanonicalizeMode :: Response { max_input_universe } => { for var in var_kinds . iter_mut () { let uv = var . universe () ; let new_uv = ty :: UniverseIndex :: from (uv . index () . saturating_sub (max_input_universe . index ()) ,) ; * var = var . with_updated_universe (new_uv) ; } let max_universe = var_kinds . iter () . map (| kind | kind . universe ()) . max () . unwrap_or (ty :: UniverseIndex :: ROOT) ; let var_kinds = self . delegate . cx () . mk_canonical_var_kinds (& var_kinds) ; (max_universe , var_kinds) } } } fn inner_fold_ty (& mut self , t : I :: Ty) -> I :: Ty { let kind = match t . kind () { ty :: Infer (i) => match i { ty :: TyVar (vid) => { debug_assert_eq ! (self . delegate . opportunistic_resolve_ty_var (vid) , t , "ty vid should have been resolved fully before canonicalization") ; let sub_root = self . get_or_insert_sub_root (vid) ; let ui = match self . canonicalize_mode { CanonicalizeMode :: Input { .. } => ty :: UniverseIndex :: ROOT , CanonicalizeMode :: Response { .. } => self . delegate . universe_of_ty (vid) . unwrap_or_else (| | panic ! ("ty var should have been resolved: {t:?}")) , } ; CanonicalVarKind :: Ty { ui , sub_root } } ty :: IntVar (vid) => { debug_assert_eq ! (self . delegate . opportunistic_resolve_int_var (vid) , t , "ty vid should have been resolved fully before canonicalization") ; CanonicalVarKind :: Int } ty :: FloatVar (vid) => { debug_assert_eq ! (self . delegate . opportunistic_resolve_float_var (vid) , t , "ty vid should have been resolved fully before canonicalization") ; CanonicalVarKind :: Float } ty :: FreshTy (_) | ty :: FreshIntTy (_) | ty :: FreshFloatTy (_) => { panic ! ("fresh vars not expected in canonicalization") } } , ty :: Placeholder (placeholder) => match self . canonicalize_mode { CanonicalizeMode :: Input { .. } => CanonicalVarKind :: PlaceholderTy (PlaceholderLike :: new_anon (ty :: UniverseIndex :: ROOT , self . variables . len () . into ()) ,) , CanonicalizeMode :: Response { .. } => CanonicalVarKind :: PlaceholderTy (placeholder) , } , ty :: Param (_) => match self . canonicalize_mode { CanonicalizeMode :: Input { .. } => CanonicalVarKind :: PlaceholderTy (PlaceholderLike :: new_anon (ty :: UniverseIndex :: ROOT , self . variables . len () . into ()) ,) , CanonicalizeMode :: Response { .. } => panic ! ("param ty in response: {t:?}") , } , ty :: Bool | ty :: Char | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Adt (_ , _) | ty :: Foreign (_) | ty :: Str | ty :: Array (_ , _) | ty :: Slice (_) | ty :: RawPtr (_ , _) | ty :: Ref (_ , _ , _) | ty :: Pat (_ , _) | ty :: FnDef (_ , _) | ty :: FnPtr (..) | ty :: UnsafeBinder (_) | ty :: Dynamic (_ , _ , _) | ty :: Closure (..) | ty :: CoroutineClosure (..) | ty :: Coroutine (_ , _) | ty :: CoroutineWitness (..) | ty :: Never | ty :: Tuple (_) | ty :: Alias (_ , _) | ty :: Bound (_ , _) | ty :: Error (_) => { return if t . has_type_flags (NEEDS_CANONICAL) { ensure_sufficient_stack (| | t . super_fold_with (self)) } else { t } ; } } ; let var = self . get_or_insert_bound_var (t , kind) ; Ty :: new_anon_bound (self . cx () , self . binder_index , var) } }}}
mkitem!{mkimpl!{impl < D : SolverDelegate < Interner = I > , I : Interner > TypeFolder < I > for Canonicalizer < '_ , D , I > { fn cx (& self) -> I { self . delegate . cx () } fn fold_binder < T > (& mut self , t : ty :: Binder < I , T >) -> ty :: Binder < I , T > where T : TypeFoldable < I > , { self . binder_index . shift_in (1) ; let t = t . super_fold_with (self) ; self . binder_index . shift_out (1) ; t } fn fold_region (& mut self , r : I :: Region) -> I :: Region { let kind = match r . kind () { ty :: ReBound (..) => return r , ty :: ReStatic => match self . canonicalize_mode { CanonicalizeMode :: Input (CanonicalizeInputKind :: Predicate { .. }) => { CanonicalVarKind :: Region (ty :: UniverseIndex :: ROOT) } CanonicalizeMode :: Input (CanonicalizeInputKind :: ParamEnv) | CanonicalizeMode :: Response { .. } => return r , } , ty :: ReErased | ty :: ReError (_) => match self . canonicalize_mode { CanonicalizeMode :: Input (_) => CanonicalVarKind :: Region (ty :: UniverseIndex :: ROOT) , CanonicalizeMode :: Response { .. } => return r , } , ty :: ReEarlyParam (_) | ty :: ReLateParam (_) => match self . canonicalize_mode { CanonicalizeMode :: Input (_) => CanonicalVarKind :: Region (ty :: UniverseIndex :: ROOT) , CanonicalizeMode :: Response { .. } => { panic ! ("unexpected region in response: {r:?}") } } , ty :: RePlaceholder (placeholder) => match self . canonicalize_mode { CanonicalizeMode :: Input (_) => CanonicalVarKind :: Region (ty :: UniverseIndex :: ROOT) , CanonicalizeMode :: Response { max_input_universe } => { if max_input_universe . can_name (placeholder . universe ()) { panic ! ("new placeholder in universe {max_input_universe:?}: {r:?}") ; } CanonicalVarKind :: PlaceholderRegion (placeholder) } } , ty :: ReVar (vid) => { debug_assert_eq ! (self . delegate . opportunistic_resolve_lt_var (vid) , r , "region vid should have been resolved fully before canonicalization") ; match self . canonicalize_mode { CanonicalizeMode :: Input (_) => CanonicalVarKind :: Region (ty :: UniverseIndex :: ROOT) , CanonicalizeMode :: Response { .. } => { CanonicalVarKind :: Region (self . delegate . universe_of_lt (vid) . unwrap ()) } } } } ; let var = self . get_or_insert_bound_var (r , kind) ; Region :: new_anon_bound (self . cx () , self . binder_index , var) } fn fold_ty (& mut self , t : I :: Ty) -> I :: Ty { if let Some (& ty) = self . cache . get (& (self . binder_index , t)) { ty } else { let res = self . inner_fold_ty (t) ; let old = self . cache . insert ((self . binder_index , t) , res) ; assert_eq ! (old , None) ; res } } fn fold_const (& mut self , c : I :: Const) -> I :: Const { let kind = match c . kind () { ty :: ConstKind :: Infer (i) => match i { ty :: InferConst :: Var (vid) => { debug_assert_eq ! (self . delegate . opportunistic_resolve_ct_var (vid) , c , "const vid should have been resolved fully before canonicalization") ; match self . canonicalize_mode { CanonicalizeMode :: Input { .. } => { CanonicalVarKind :: Const (ty :: UniverseIndex :: ROOT) } CanonicalizeMode :: Response { .. } => { CanonicalVarKind :: Const (self . delegate . universe_of_ct (vid) . unwrap ()) } } } ty :: InferConst :: Fresh (_) => todo ! () , } , ty :: ConstKind :: Placeholder (placeholder) => match self . canonicalize_mode { CanonicalizeMode :: Input { .. } => CanonicalVarKind :: PlaceholderConst (PlaceholderLike :: new_anon (ty :: UniverseIndex :: ROOT , self . variables . len () . into ()) ,) , CanonicalizeMode :: Response { .. } => { CanonicalVarKind :: PlaceholderConst (placeholder) } } , ty :: ConstKind :: Param (_) => match self . canonicalize_mode { CanonicalizeMode :: Input { .. } => CanonicalVarKind :: PlaceholderConst (PlaceholderLike :: new_anon (ty :: UniverseIndex :: ROOT , self . variables . len () . into ()) ,) , CanonicalizeMode :: Response { .. } => panic ! ("param ty in response: {c:?}") , } , ty :: ConstKind :: Bound (_ , _) | ty :: ConstKind :: Unevaluated (_) | ty :: ConstKind :: Value (_) | ty :: ConstKind :: Error (_) | ty :: ConstKind :: Expr (_) => { return if c . has_type_flags (NEEDS_CANONICAL) { c . super_fold_with (self) } else { c } ; } } ; let var = self . get_or_insert_bound_var (c , kind) ; Const :: new_anon_bound (self . cx () , self . binder_index , var) } fn fold_predicate (& mut self , p : I :: Predicate) -> I :: Predicate { if p . flags () . intersects (NEEDS_CANONICAL) { p . super_fold_with (self) } else { p } } fn fold_clauses (& mut self , c : I :: Clauses) -> I :: Clauses { match self . canonicalize_mode { CanonicalizeMode :: Input (CanonicalizeInputKind :: ParamEnv) | CanonicalizeMode :: Response { max_input_universe : _ } => { } CanonicalizeMode :: Input (CanonicalizeInputKind :: Predicate { .. }) => { panic ! ("erasing 'static in env") } } if c . flags () . intersects (NEEDS_CANONICAL) { c . super_fold_with (self) } else { c } } }}}