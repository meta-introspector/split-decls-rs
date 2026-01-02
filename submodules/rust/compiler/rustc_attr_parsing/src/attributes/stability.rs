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
mkuse!{use std :: num :: NonZero ;}
mkuse!{use rustc_errors :: ErrorGuaranteed ;}
mkuse!{use rustc_hir :: { DefaultBodyStability , MethodKind , PartialConstStability , Stability , StabilityLevel , StableSince , Target , UnstableReason , VERSION_PLACEHOLDER , } ;}
mkuse!{use super :: prelude :: * ;}
mkuse!{use super :: util :: parse_version ;}
mkuse!{use crate :: session_diagnostics :: { self , UnsupportedLiteralReason } ;}
mkitem!{macro_rules ! reject_outside_std { ($ cx : ident) => { if !$ cx . features () . staged_api () { $ cx . emit_err (session_diagnostics :: StabilityOutsideStd { span : $ cx . attr_span }) ; return ; } } ; }}
mkitem!{const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Struct) , Allow (Target :: Enum) , Allow (Target :: Union) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: Trait { body : false })) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) , Allow (Target :: Impl { of_trait : false }) , Allow (Target :: Impl { of_trait : true }) , Allow (Target :: MacroDef) , Allow (Target :: Crate) , Allow (Target :: Mod) , Allow (Target :: Use) , Allow (Target :: Const) , Allow (Target :: AssocConst) , Allow (Target :: AssocTy) , Allow (Target :: Trait) , Allow (Target :: TraitAlias) , Allow (Target :: TyAlias) , Allow (Target :: Variant) , Allow (Target :: Field) , Allow (Target :: Param) , Allow (Target :: Static) , Allow (Target :: ForeignFn) , Allow (Target :: ForeignStatic) , Allow (Target :: ExternCrate) ,]) ;}
mkitem!{mkstruct!{# [derive (Default)] pub (crate) struct StabilityParser { allowed_through_unstable_modules : Option < Symbol > , stability : Option < (Stability , Span) > , }}}
mkitem!{mkimpl!{impl StabilityParser { # [doc = " Checks, and emits an error when a stability (or unstability) was already set, which would be a duplicate."] fn check_duplicate < S : Stage > (& self , cx : & AcceptContext < '_ , '_ , S >) -> bool { if let Some ((_ , _)) = self . stability { cx . emit_err (session_diagnostics :: MultipleStabilityLevels { span : cx . attr_span }) ; true } else { false } } }}}
mkitem!{mkimpl!{impl < S : Stage > AttributeParser < S > for StabilityParser { const ATTRIBUTES : AcceptMapping < Self , S > = & [(& [sym :: stable] , template ! (List : & [r#"feature = "name", since = "version""#]) , | this , cx , args | { reject_outside_std ! (cx) ; if ! this . check_duplicate (cx) && let Some ((feature , level)) = parse_stability (cx , args) { this . stability = Some ((Stability { level , feature } , cx . attr_span)) ; } } ,) , (& [sym :: unstable] , template ! (List : & [r#"feature = "name", reason = "...", issue = "N""#]) , | this , cx , args | { reject_outside_std ! (cx) ; if ! this . check_duplicate (cx) && let Some ((feature , level)) = parse_unstability (cx , args) { this . stability = Some ((Stability { level , feature } , cx . attr_span)) ; } } ,) , (& [sym :: rustc_allowed_through_unstable_modules] , template ! (NameValueStr : "deprecation message") , | this , cx , args | { reject_outside_std ! (cx) ; let Some (nv) = args . name_value () else { cx . expected_name_value (cx . attr_span , None) ; return ; } ; let Some (value_str) = nv . value_as_str () else { cx . expected_string_literal (nv . value_span , Some (nv . value_as_lit ())) ; return ; } ; this . allowed_through_unstable_modules = Some (value_str) ; } ,) ,] ; const ALLOWED_TARGETS : AllowedTargets = ALLOWED_TARGETS ; fn finalize (mut self , cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { if let Some (atum) = self . allowed_through_unstable_modules { if let Some ((Stability { level : StabilityLevel :: Stable { ref mut allowed_through_unstable_modules , .. } , .. } , _ ,)) = self . stability { * allowed_through_unstable_modules = Some (atum) ; } else { cx . dcx () . emit_err (session_diagnostics :: RustcAllowedUnstablePairing { span : cx . target_span , }) ; } } if let Some ((Stability { level : StabilityLevel :: Stable { .. } , .. } , _)) = self . stability { for other_attr in cx . all_attrs { if other_attr . word_is (sym :: unstable_feature_bound) { cx . emit_err (session_diagnostics :: UnstableFeatureBoundIncompatibleStability { span : cx . target_span , }) ; } } } let (stability , span) = self . stability ? ; Some (AttributeKind :: Stability { stability , span }) } }}}
mkitem!{mkstruct!{# [derive (Default)] pub (crate) struct BodyStabilityParser { stability : Option < (DefaultBodyStability , Span) > , }}}
mkitem!{mkimpl!{impl < S : Stage > AttributeParser < S > for BodyStabilityParser { const ATTRIBUTES : AcceptMapping < Self , S > = & [(& [sym :: rustc_default_body_unstable] , template ! (List : & [r#"feature = "name", reason = "...", issue = "N""#]) , | this , cx , args | { reject_outside_std ! (cx) ; if this . stability . is_some () { cx . dcx () . emit_err (session_diagnostics :: MultipleStabilityLevels { span : cx . attr_span }) ; } else if let Some ((feature , level)) = parse_unstability (cx , args) { this . stability = Some ((DefaultBodyStability { level , feature } , cx . attr_span)) ; } } ,)] ; const ALLOWED_TARGETS : AllowedTargets = ALLOWED_TARGETS ; fn finalize (self , _cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { let (stability , span) = self . stability ? ; Some (AttributeKind :: BodyStability { stability , span }) } }}}
mkitem!{mkstruct!{pub (crate) struct ConstStabilityIndirectParser ;}}
mkitem!{mkimpl!{impl < S : Stage > NoArgsAttributeParser < S > for ConstStabilityIndirectParser { const PATH : & [Symbol] = & [sym :: rustc_const_stable_indirect] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Ignore ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) ,]) ; const CREATE : fn (Span) -> AttributeKind = | _ | AttributeKind :: ConstStabilityIndirect ; }}}
mkitem!{mkstruct!{# [derive (Default)] pub (crate) struct ConstStabilityParser { promotable : bool , stability : Option < (PartialConstStability , Span) > , }}}
mkitem!{mkimpl!{impl ConstStabilityParser { # [doc = " Checks, and emits an error when a stability (or unstability) was already set, which would be a duplicate."] fn check_duplicate < S : Stage > (& self , cx : & AcceptContext < '_ , '_ , S >) -> bool { if let Some ((_ , _)) = self . stability { cx . emit_err (session_diagnostics :: MultipleStabilityLevels { span : cx . attr_span }) ; true } else { false } } }}}
mkitem!{mkimpl!{impl < S : Stage > AttributeParser < S > for ConstStabilityParser { const ATTRIBUTES : AcceptMapping < Self , S > = & [(& [sym :: rustc_const_stable] , template ! (List : & [r#"feature = "name""#]) , | this , cx , args | { reject_outside_std ! (cx) ; if ! this . check_duplicate (cx) && let Some ((feature , level)) = parse_stability (cx , args) { this . stability = Some ((PartialConstStability { level , feature , promotable : false } , cx . attr_span ,)) ; } } ,) , (& [sym :: rustc_const_unstable] , template ! (List : & [r#"feature = "name""#]) , | this , cx , args | { reject_outside_std ! (cx) ; if ! this . check_duplicate (cx) && let Some ((feature , level)) = parse_unstability (cx , args) { this . stability = Some ((PartialConstStability { level , feature , promotable : false } , cx . attr_span ,)) ; } } ,) , (& [sym :: rustc_promotable] , template ! (Word) , | this , cx , _ | { reject_outside_std ! (cx) ; this . promotable = true ; }) ,] ; const ALLOWED_TARGETS : AllowedTargets = ALLOWED_TARGETS ; fn finalize (mut self , cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { if self . promotable { if let Some ((ref mut stab , _)) = self . stability { stab . promotable = true ; } else { cx . dcx () . emit_err (session_diagnostics :: RustcPromotablePairing { span : cx . target_span }) ; } } let (stability , span) = self . stability ? ; Some (AttributeKind :: ConstStability { stability , span }) } }}}

macro_rules! insert_value_into_option_or_error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function insert_value_into_option_or_error in module {}", module_path!());
    };
}

mkfn!{
    insert_value_into_option_or_error_introspect!();
    # [doc = " Tries to insert the value of a `key = value` meta item into an option."] # [doc = ""] # [doc = " Emits an error when either the option was already Some, or the arguments weren't of form"] # [doc = " `name = value`"] fn insert_value_into_option_or_error < S : Stage > (cx : & AcceptContext < '_ , '_ , S > , param : & MetaItemParser < '_ > , item : & mut Option < Symbol > , name : Ident ,) -> Option < () > { if item . is_some () { cx . duplicate_key (name . span , name . name) ; None } else if let Some (v) = param . args () . name_value () && let Some (s) = v . value_as_str () { * item = Some (s) ; Some (()) } else { cx . expected_name_value (param . span () , Some (name . name)) ; None } }
}

macro_rules! parse_stability_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_stability in module {}", module_path!());
    };
}

mkfn!{
    parse_stability_introspect!();
    # [doc = " Read the content of a `stable`/`rustc_const_stable` attribute, and return the feature name and"] # [doc = " its stability information."] pub (crate) fn parse_stability < S : Stage > (cx : & AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ > ,) -> Option < (Symbol , StabilityLevel) > { let mut feature = None ; let mut since = None ; let ArgParser :: List (list) = args else { cx . expected_list (cx . attr_span) ; return None ; } ; for param in list . mixed () { let param_span = param . span () ; let Some (param) = param . meta_item () else { cx . emit_err (session_diagnostics :: UnsupportedLiteral { span : param_span , reason : UnsupportedLiteralReason :: Generic , is_bytestr : false , start_point_span : cx . sess () . source_map () . start_point (param_span) , }) ; return None ; } ; let word = param . path () . word () ; match word . map (| i | i . name) { Some (sym :: feature) => { insert_value_into_option_or_error (cx , & param , & mut feature , word . unwrap ()) ? } Some (sym :: since) => { insert_value_into_option_or_error (cx , & param , & mut since , word . unwrap ()) ? } _ => { cx . emit_err (session_diagnostics :: UnknownMetaItem { span : param_span , item : param . path () . to_string () , expected : & ["feature" , "since"] , }) ; return None ; } } } let feature = match feature { Some (feature) if rustc_lexer :: is_ident (feature . as_str ()) => Ok (feature) , Some (_bad_feature) => { Err (cx . emit_err (session_diagnostics :: NonIdentFeature { span : cx . attr_span })) } None => Err (cx . emit_err (session_diagnostics :: MissingFeature { span : cx . attr_span })) , } ; let since = if let Some (since) = since { if since . as_str () == VERSION_PLACEHOLDER { StableSince :: Current } else if let Some (version) = parse_version (since) { StableSince :: Version (version) } else { let err = cx . emit_err (session_diagnostics :: InvalidSince { span : cx . attr_span }) ; StableSince :: Err (err) } } else { let err = cx . emit_err (session_diagnostics :: MissingSince { span : cx . attr_span }) ; StableSince :: Err (err) } ; match feature { Ok (feature) => { let level = StabilityLevel :: Stable { since , allowed_through_unstable_modules : None } ; Some ((feature , level)) } Err (ErrorGuaranteed { .. }) => None , } }
}

macro_rules! parse_unstability_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_unstability in module {}", module_path!());
    };
}

mkfn!{
    parse_unstability_introspect!();
    # [doc = " attribute, and return the feature name and its stability information."] pub (crate) fn parse_unstability < S : Stage > (cx : & AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ > ,) -> Option < (Symbol , StabilityLevel) > { let mut feature = None ; let mut reason = None ; let mut issue = None ; let mut issue_num = None ; let mut is_soft = false ; let mut implied_by = None ; let mut old_name = None ; let ArgParser :: List (list) = args else { cx . expected_list (cx . attr_span) ; return None ; } ; for param in list . mixed () { let Some (param) = param . meta_item () else { cx . emit_err (session_diagnostics :: UnsupportedLiteral { span : param . span () , reason : UnsupportedLiteralReason :: Generic , is_bytestr : false , start_point_span : cx . sess () . source_map () . start_point (param . span ()) , }) ; return None ; } ; let word = param . path () . word () ; match word . map (| i | i . name) { Some (sym :: feature) => { insert_value_into_option_or_error (cx , & param , & mut feature , word . unwrap ()) ? } Some (sym :: reason) => { insert_value_into_option_or_error (cx , & param , & mut reason , word . unwrap ()) ? } Some (sym :: issue) => { insert_value_into_option_or_error (cx , & param , & mut issue , word . unwrap ()) ? ; issue_num = match issue . unwrap () . as_str () { "none" => None , issue_str => match issue_str . parse :: < NonZero < u32 > > () { Ok (num) => Some (num) , Err (err) => { cx . emit_err (session_diagnostics :: InvalidIssueString { span : param . span () , cause : session_diagnostics :: InvalidIssueStringCause :: from_int_error_kind (param . args () . name_value () . unwrap () . value_span , err . kind () ,) , } ,) ; return None ; } } , } ; } Some (sym :: soft) => { if let Err (span) = args . no_args () { cx . emit_err (session_diagnostics :: SoftNoArgs { span }) ; } is_soft = true ; } Some (sym :: implied_by) => { insert_value_into_option_or_error (cx , & param , & mut implied_by , word . unwrap ()) ? } Some (sym :: old_name) => { insert_value_into_option_or_error (cx , & param , & mut old_name , word . unwrap ()) ? } _ => { cx . emit_err (session_diagnostics :: UnknownMetaItem { span : param . span () , item : param . path () . to_string () , expected : & ["feature" , "reason" , "issue" , "soft" , "implied_by" , "old_name"] , }) ; return None ; } } } let feature = match feature { Some (feature) if rustc_lexer :: is_ident (feature . as_str ()) => Ok (feature) , Some (_bad_feature) => { Err (cx . emit_err (session_diagnostics :: NonIdentFeature { span : cx . attr_span })) } None => Err (cx . emit_err (session_diagnostics :: MissingFeature { span : cx . attr_span })) , } ; let issue = issue . ok_or_else (| | cx . emit_err (session_diagnostics :: MissingIssue { span : cx . attr_span })) ; match (feature , issue) { (Ok (feature) , Ok (_)) => { let level = StabilityLevel :: Unstable { reason : UnstableReason :: from_opt_reason (reason) , issue : issue_num , is_soft , implied_by , old_name , } ; Some ((feature , level)) } (Err (ErrorGuaranteed { .. }) , _) | (_ , Err (ErrorGuaranteed { .. })) => None , } }
}