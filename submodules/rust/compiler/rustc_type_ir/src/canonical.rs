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
mkuse!{use std :: fmt ;}
mkuse!{use std :: ops :: Index ;}
mkuse!{use arrayvec :: ArrayVec ;}
mkuse!{use derive_where :: derive_where ;}
mkuse!{# [cfg (feature = "nightly")] use rustc_macros :: { Decodable_NoContext , Encodable_NoContext , HashStable_NoContext } ;}
mkuse!{use rustc_type_ir_macros :: { Lift_Generic , TypeFoldable_Generic , TypeVisitable_Generic } ;}
mkuse!{use crate :: data_structures :: HashMap ;}
mkuse!{use crate :: inherent :: * ;}
mkuse!{use crate :: { self as ty , Interner , TypingMode , UniverseIndex } ;}
mkitem!{mkstruct!{# [derive_where (Clone , Hash , PartialEq , Debug ; I : Interner , V)] # [derive_where (Copy ; I : Interner , V : Copy)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub struct CanonicalQueryInput < I : Interner , V > { pub canonical : Canonical < I , V > , pub typing_mode : TypingMode < I > , }}}
mkitem!{mkimpl!{impl < I : Interner , V : Eq > Eq for CanonicalQueryInput < I , V > { }}}
mkitem!{mkstruct!{# [doc = " A \"canonicalized\" type `V` is one where all free inference"] # [doc = " variables have been rewritten to \"canonical vars\". These are"] # [doc = " numbered starting from 0 in order of first appearance."] # [derive_where (Clone , Hash , PartialEq , Debug ; I : Interner , V)] # [derive_where (Copy ; I : Interner , V : Copy)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub struct Canonical < I : Interner , V > { pub value : V , pub max_universe : UniverseIndex , pub variables : I :: CanonicalVarKinds , }}}
mkitem!{mkimpl!{impl < I : Interner , V : Eq > Eq for Canonical < I , V > { }}}
mkitem!{mkimpl!{impl < I : Interner , V > Canonical < I , V > { # [doc = " Allows you to map the `value` of a canonical while keeping the"] # [doc = " same set of bound variables."] # [doc = ""] # [doc = " **WARNING:** This function is very easy to mis-use, hence the"] # [doc = " name!  In particular, the new value `W` must use all **the"] # [doc = " same type/region variables** in **precisely the same order**"] # [doc = " as the original! (The ordering is defined by the"] # [doc = " `TypeFoldable` implementation of the type in question.)"] # [doc = ""] # [doc = " An example of a **correct** use of this:"] # [doc = ""] # [doc = " ```rust,ignore (not real code)"] # [doc = " let a: Canonical<I, T> = ...;"] # [doc = " let b: Canonical<I, (T,)> = a.unchecked_map(|v| (v, ));"] # [doc = " ```"] # [doc = ""] # [doc = " An example of an **incorrect** use of this:"] # [doc = ""] # [doc = " ```rust,ignore (not real code)"] # [doc = " let a: Canonical<I, T> = ...;"] # [doc = " let ty: Ty<I> = ...;"] # [doc = " let b: Canonical<I, (T, Ty<I>)> = a.unchecked_map(|v| (v, ty));"] # [doc = " ```"] pub fn unchecked_map < W > (self , map_op : impl FnOnce (V) -> W) -> Canonical < I , W > { let Canonical { max_universe , variables , value } = self ; Canonical { max_universe , variables , value : map_op (value) } } }}}
mkitem!{mkimpl!{impl < I : Interner , V : fmt :: Display > fmt :: Display for Canonical < I , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let Self { value , max_universe , variables } = self ; write ! (f , "Canonical {{ value: {value}, max_universe: {max_universe:?}, variables: {variables:?} }}" ,) } }}}
mkitem!{mkenum!{# [doc = " Information about a canonical variable that is included with the"] # [doc = " canonical value. This is sufficient information for code to create"] # [doc = " a copy of the canonical value in some other inference context,"] # [doc = " with fresh inference variables replacing the canonical values."] # [derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub enum CanonicalVarKind < I : Interner > { # [doc = " General type variable `?T` that can be unified with arbitrary types."] # [doc = ""] # [doc = " We also store the index of the first type variable which is sub-unified"] # [doc = " with this one. If there is no inference variable related to this one,"] # [doc = " its `sub_root` just points to itself."] Ty { ui : UniverseIndex , sub_root : ty :: BoundVar } , # [doc = " Integral type variable `?I` (that can only be unified with integral types)."] Int , # [doc = " Floating-point type variable `?F` (that can only be unified with float types)."] Float , # [doc = " A \"placeholder\" that represents \"any type\"."] PlaceholderTy (I :: PlaceholderTy) , # [doc = " Region variable `'?R`."] Region (UniverseIndex) , # [doc = " A \"placeholder\" that represents \"any region\". Created when you"] # [doc = " are solving a goal like `for<'a> T: Foo<'a>` to represent the"] # [doc = " bound region `'a`."] PlaceholderRegion (I :: PlaceholderRegion) , # [doc = " Some kind of const inference variable."] Const (UniverseIndex) , # [doc = " A \"placeholder\" that represents \"any const\"."] PlaceholderConst (I :: PlaceholderConst) , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for CanonicalVarKind < I > { }}}
mkitem!{mkimpl!{impl < I : Interner > CanonicalVarKind < I > { pub fn universe (self) -> UniverseIndex { match self { CanonicalVarKind :: Ty { ui , sub_root : _ } => ui , CanonicalVarKind :: Region (ui) => ui , CanonicalVarKind :: Const (ui) => ui , CanonicalVarKind :: PlaceholderTy (placeholder) => placeholder . universe () , CanonicalVarKind :: PlaceholderRegion (placeholder) => placeholder . universe () , CanonicalVarKind :: PlaceholderConst (placeholder) => placeholder . universe () , CanonicalVarKind :: Float | CanonicalVarKind :: Int => UniverseIndex :: ROOT , } } # [doc = " Replaces the universe of this canonical variable with `ui`."] # [doc = ""] # [doc = " In case this is a float or int variable, this causes an ICE if"] # [doc = " the updated universe is not the root."] pub fn with_updated_universe (self , ui : UniverseIndex) -> CanonicalVarKind < I > { match self { CanonicalVarKind :: Ty { ui : _ , sub_root } => CanonicalVarKind :: Ty { ui , sub_root } , CanonicalVarKind :: Region (_) => CanonicalVarKind :: Region (ui) , CanonicalVarKind :: Const (_) => CanonicalVarKind :: Const (ui) , CanonicalVarKind :: PlaceholderTy (placeholder) => { CanonicalVarKind :: PlaceholderTy (placeholder . with_updated_universe (ui)) } CanonicalVarKind :: PlaceholderRegion (placeholder) => { CanonicalVarKind :: PlaceholderRegion (placeholder . with_updated_universe (ui)) } CanonicalVarKind :: PlaceholderConst (placeholder) => { CanonicalVarKind :: PlaceholderConst (placeholder . with_updated_universe (ui)) } CanonicalVarKind :: Int | CanonicalVarKind :: Float => { assert_eq ! (ui , UniverseIndex :: ROOT) ; self } } } pub fn is_existential (self) -> bool { match self { CanonicalVarKind :: Ty { .. } | CanonicalVarKind :: Int | CanonicalVarKind :: Float | CanonicalVarKind :: Region (_) | CanonicalVarKind :: Const (_) => true , CanonicalVarKind :: PlaceholderTy (_) | CanonicalVarKind :: PlaceholderRegion (..) | CanonicalVarKind :: PlaceholderConst (_) => false , } } pub fn is_region (self) -> bool { match self { CanonicalVarKind :: Region (_) | CanonicalVarKind :: PlaceholderRegion (_) => true , CanonicalVarKind :: Ty { .. } | CanonicalVarKind :: Int | CanonicalVarKind :: Float | CanonicalVarKind :: PlaceholderTy (_) | CanonicalVarKind :: Const (_) | CanonicalVarKind :: PlaceholderConst (_) => false , } } pub fn expect_placeholder_index (self) -> usize { match self { CanonicalVarKind :: Ty { .. } | CanonicalVarKind :: Int | CanonicalVarKind :: Float | CanonicalVarKind :: Region (_) | CanonicalVarKind :: Const (_) => { panic ! ("expected placeholder: {self:?}") } CanonicalVarKind :: PlaceholderRegion (placeholder) => placeholder . var () . as_usize () , CanonicalVarKind :: PlaceholderTy (placeholder) => placeholder . var () . as_usize () , CanonicalVarKind :: PlaceholderConst (placeholder) => placeholder . var () . as_usize () , } } }}}
mkitem!{mkstruct!{# [doc = " A set of values corresponding to the canonical variables from some"] # [doc = " `Canonical`. You can give these values to"] # [doc = " `canonical_value.instantiate` to instantiate them into the canonical"] # [doc = " value at the right places."] # [doc = ""] # [doc = " When you canonicalize a value `V`, you get back one of these"] # [doc = " vectors with the original values that were replaced by canonical"] # [doc = " variables. You will need to supply it later to instantiate the"] # [doc = " canonicalized query response."] # [derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] pub struct CanonicalVarValues < I : Interner > { pub var_values : I :: GenericArgs , }}}
mkitem!{mkimpl!{impl < I : Interner > Eq for CanonicalVarValues < I > { }}}
mkitem!{mkimpl!{impl < I : Interner > CanonicalVarValues < I > { pub fn is_identity (& self) -> bool { self . var_values . iter () . enumerate () . all (| (bv , arg) | match arg . kind () { ty :: GenericArgKind :: Lifetime (r) => { matches ! (r . kind () , ty :: ReBound (ty :: INNERMOST , br) if br . var () . as_usize () == bv) } ty :: GenericArgKind :: Type (ty) => { matches ! (ty . kind () , ty :: Bound (ty :: INNERMOST , bt) if bt . var () . as_usize () == bv) } ty :: GenericArgKind :: Const (ct) => { matches ! (ct . kind () , ty :: ConstKind :: Bound (ty :: INNERMOST , bc) if bc . var () . as_usize () == bv) } }) } pub fn is_identity_modulo_regions (& self) -> bool { let mut var = ty :: BoundVar :: ZERO ; for arg in self . var_values . iter () { match arg . kind () { ty :: GenericArgKind :: Lifetime (r) => { if matches ! (r . kind () , ty :: ReBound (ty :: INNERMOST , br) if var == br . var ()) { var = var + 1 ; } else { } } ty :: GenericArgKind :: Type (ty) => { if matches ! (ty . kind () , ty :: Bound (ty :: INNERMOST , bt) if var == bt . var ()) { var = var + 1 ; } else { return false ; } } ty :: GenericArgKind :: Const (ct) => { if matches ! (ct . kind () , ty :: ConstKind :: Bound (ty :: INNERMOST , bc) if var == bc . var ()) { var = var + 1 ; } else { return false ; } } } } true } pub fn make_identity (cx : I , infos : I :: CanonicalVarKinds) -> CanonicalVarValues < I > { CanonicalVarValues { var_values : cx . mk_args_from_iter (infos . iter () . enumerate () . map (| (i , kind) | -> I :: GenericArg { match kind { CanonicalVarKind :: Ty { .. } | CanonicalVarKind :: Int | CanonicalVarKind :: Float | CanonicalVarKind :: PlaceholderTy (_) => { Ty :: new_anon_bound (cx , ty :: INNERMOST , ty :: BoundVar :: from_usize (i)) . into () } CanonicalVarKind :: Region (_) | CanonicalVarKind :: PlaceholderRegion (_) => { Region :: new_anon_bound (cx , ty :: INNERMOST , ty :: BoundVar :: from_usize (i)) . into () } CanonicalVarKind :: Const (_) | CanonicalVarKind :: PlaceholderConst (_) => { Const :: new_anon_bound (cx , ty :: INNERMOST , ty :: BoundVar :: from_usize (i)) . into () } } } ,)) , } } # [doc = " Creates dummy var values which should not be used in a"] # [doc = " canonical response."] pub fn dummy () -> CanonicalVarValues < I > { CanonicalVarValues { var_values : Default :: default () } } pub fn instantiate (cx : I , variables : I :: CanonicalVarKinds , mut f : impl FnMut (& [I :: GenericArg] , CanonicalVarKind < I >) -> I :: GenericArg ,) -> CanonicalVarValues < I > { if variables . len () <= 4 { let mut var_values = ArrayVec :: < _ , 4 > :: new () ; for info in variables . iter () { var_values . push (f (& var_values , info)) ; } CanonicalVarValues { var_values : cx . mk_args (& var_values) } } else { CanonicalVarValues :: instantiate_cold (cx , variables , f) } } # [cold] fn instantiate_cold (cx : I , variables : I :: CanonicalVarKinds , mut f : impl FnMut (& [I :: GenericArg] , CanonicalVarKind < I >) -> I :: GenericArg ,) -> CanonicalVarValues < I > { let mut var_values = Vec :: with_capacity (variables . len ()) ; for info in variables . iter () { var_values . push (f (& var_values , info)) ; } CanonicalVarValues { var_values : cx . mk_args (& var_values) } } # [inline] pub fn len (& self) -> usize { self . var_values . len () } }}}
mkitem!{mkimpl!{impl < 'a , I : Interner > IntoIterator for & 'a CanonicalVarValues < I > { type Item = I :: GenericArg ; type IntoIter = < I :: GenericArgs as SliceLike > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { self . var_values . iter () } }}}
mkitem!{mkimpl!{impl < I : Interner > Index < ty :: BoundVar > for CanonicalVarValues < I > { type Output = I :: GenericArg ; fn index (& self , value : ty :: BoundVar) -> & I :: GenericArg { & self . var_values . as_slice () [value . as_usize ()] } }}}
mkitem!{mkstruct!{# [derive_where (Clone , Debug ; I : Interner)] pub struct CanonicalParamEnvCacheEntry < I : Interner > { pub param_env : I :: ParamEnv , pub variables : Vec < I :: GenericArg > , pub variable_lookup_table : HashMap < I :: GenericArg , usize > , pub var_kinds : Vec < CanonicalVarKind < I > > , }}}