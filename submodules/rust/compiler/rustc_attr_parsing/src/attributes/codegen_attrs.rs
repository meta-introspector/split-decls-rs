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
mkuse!{use rustc_hir :: attrs :: { CoverageAttrKind , OptimizeAttr , SanitizerSet , UsedBy } ;}
mkuse!{use rustc_session :: parse :: feature_err ;}
mkuse!{use super :: prelude :: * ;}
mkuse!{use crate :: session_diagnostics :: { NakedFunctionIncompatibleAttribute , NullOnExport } ;}
mkitem!{mkstruct!{pub (crate) struct OptimizeParser ;}}
mkitem!{mkimpl!{impl < S : Stage > SingleAttributeParser < S > for OptimizeParser { const PATH : & [Symbol] = & [sym :: optimize] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: WarnButFutureError ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Closure) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) , Allow (Target :: Method (MethodKind :: Inherent)) ,]) ; const TEMPLATE : AttributeTemplate = template ! (List : & ["size" , "speed" , "none"]) ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let Some (list) = args . list () else { cx . expected_list (cx . attr_span) ; return None ; } ; let Some (single) = list . single () else { cx . expected_single_argument (list . span) ; return None ; } ; let res = match single . meta_item () . and_then (| i | i . path () . word () . map (| i | i . name)) { Some (sym :: size) => OptimizeAttr :: Size , Some (sym :: speed) => OptimizeAttr :: Speed , Some (sym :: none) => OptimizeAttr :: DoNotOptimize , _ => { cx . expected_specific_argument (single . span () , & [sym :: size , sym :: speed , sym :: none]) ; OptimizeAttr :: Default } } ; Some (AttributeKind :: Optimize (res , cx . attr_span)) } }}}
mkitem!{mkstruct!{pub (crate) struct ColdParser ;}}
mkitem!{mkimpl!{impl < S : Stage > NoArgsAttributeParser < S > for ColdParser { const PATH : & [Symbol] = & [sym :: cold] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowListWarnRest (& [Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) , Allow (Target :: Method (MethodKind :: Trait { body : false })) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: ForeignFn) , Allow (Target :: Closure) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: Cold ; }}}
mkitem!{mkstruct!{pub (crate) struct CoverageParser ;}}
mkitem!{mkimpl!{impl < S : Stage > SingleAttributeParser < S > for CoverageParser { const PATH : & [Symbol] = & [sym :: coverage] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Closure) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Impl { of_trait : true }) , Allow (Target :: Impl { of_trait : false }) , Allow (Target :: Mod) , Allow (Target :: Crate) ,]) ; const TEMPLATE : AttributeTemplate = template ! (OneOf : & [sym :: off , sym :: on]) ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let Some (args) = args . list () else { cx . expected_specific_argument_and_list (cx . attr_span , & [sym :: on , sym :: off]) ; return None ; } ; let Some (arg) = args . single () else { cx . expected_single_argument (args . span) ; return None ; } ; let fail_incorrect_argument = | span | cx . expected_specific_argument (span , & [sym :: on , sym :: off]) ; let Some (arg) = arg . meta_item () else { fail_incorrect_argument (args . span) ; return None ; } ; let kind = match arg . path () . word_sym () { Some (sym :: off) => CoverageAttrKind :: Off , Some (sym :: on) => CoverageAttrKind :: On , None | Some (_) => { fail_incorrect_argument (arg . span ()) ; return None ; } } ; Some (AttributeKind :: Coverage (cx . attr_span , kind)) } }}}
mkitem!{mkstruct!{pub (crate) struct ExportNameParser ;}}
mkitem!{mkimpl!{impl < S : Stage > SingleAttributeParser < S > for ExportNameParser { const PATH : & [rustc_span :: Symbol] = & [sym :: export_name] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepInnermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: WarnButFutureError ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Static) , Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) , Warn (Target :: Field) , Warn (Target :: Arm) , Warn (Target :: MacroDef) , Warn (Target :: MacroCall) ,]) ; const TEMPLATE : AttributeTemplate = template ! (NameValueStr : "name") ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let Some (nv) = args . name_value () else { cx . expected_name_value (cx . attr_span , None) ; return None ; } ; let Some (name) = nv . value_as_str () else { cx . expected_string_literal (nv . value_span , Some (nv . value_as_lit ())) ; return None ; } ; if name . as_str () . contains ('\0') { cx . emit_err (NullOnExport { span : cx . attr_span }) ; return None ; } Some (AttributeKind :: ExportName { name , span : cx . attr_span }) } }}}
mkitem!{mkstruct!{# [derive (Default)] pub (crate) struct NakedParser { span : Option < Span > , }}}
mkitem!{mkimpl!{impl < S : Stage > AttributeParser < S > for NakedParser { const ATTRIBUTES : AcceptMapping < Self , S > = & [(& [sym :: naked] , template ! (Word) , | this , cx , args | { if let Err (span) = args . no_args () { cx . expected_no_args (span) ; return ; } if let Some (earlier) = this . span { let span = cx . attr_span ; cx . warn_unused_duplicate (earlier , span) ; } else { this . span = Some (cx . attr_span) ; } })] ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) , Warn (Target :: MacroCall) ,]) ; fn finalize (self , cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { const ALLOW_LIST : & [rustc_span :: Symbol] = & [sym :: cfg_trace , sym :: cfg_attr_trace , sym :: test , sym :: ignore , sym :: should_panic , sym :: bench , sym :: allow , sym :: warn , sym :: deny , sym :: forbid , sym :: deprecated , sym :: must_use , sym :: cold , sym :: export_name , sym :: link_section , sym :: linkage , sym :: no_mangle , sym :: instruction_set , sym :: repr , sym :: rustc_std_internal_symbol , sym :: rustc_align , sym :: rustc_align_static , sym :: naked , sym :: doc ,] ; let span = self . span ? ; 'outer : for other_attr in cx . all_attrs { for allowed_attr in ALLOW_LIST { if other_attr . segments () . next () . is_some_and (| i | cx . tools . contains (& i . name)) { continue 'outer ; } if other_attr . word_is (* allowed_attr) { continue 'outer ; } if other_attr . word_is (sym :: target_feature) { if ! cx . features () . naked_functions_target_feature () { feature_err (& cx . sess () , sym :: naked_functions_target_feature , other_attr . span () , "`#[target_feature(/* ... */)]` is currently unstable on `#[naked]` functions" ,) . emit () ; } continue 'outer ; } } cx . emit_err (NakedFunctionIncompatibleAttribute { span : other_attr . span () , naked_span : span , attr : other_attr . get_attribute_path () . to_string () , }) ; } Some (AttributeKind :: Naked (span)) } }}}
mkitem!{mkstruct!{pub (crate) struct TrackCallerParser ;}}
mkitem!{mkimpl!{impl < S : Stage > NoArgsAttributeParser < S > for TrackCallerParser { const PATH : & [Symbol] = & [sym :: track_caller] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) , Allow (Target :: Method (MethodKind :: Trait { body : false })) , Allow (Target :: ForeignFn) , Allow (Target :: Closure) , Warn (Target :: MacroDef) , Warn (Target :: Arm) , Warn (Target :: Field) , Warn (Target :: MacroCall) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: TrackCaller ; }}}
mkitem!{mkstruct!{pub (crate) struct NoMangleParser ;}}
mkitem!{mkimpl!{impl < S : Stage > NoArgsAttributeParser < S > for NoMangleParser { const PATH : & [Symbol] = & [sym :: no_mangle] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowListWarnRest (& [Allow (Target :: Fn) , Allow (Target :: Static) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: TraitImpl)) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: NoMangle ; }}}
mkitem!{mkstruct!{# [derive (Default)] pub (crate) struct UsedParser { first_compiler : Option < Span > , first_linker : Option < Span > , }}}
mkitem!{mkimpl!{impl < S : Stage > AttributeParser < S > for UsedParser { const ATTRIBUTES : AcceptMapping < Self , S > = & [(& [sym :: used] , template ! (Word , List : & ["compiler" , "linker"]) , | group : & mut Self , cx , args | { let used_by = match args { ArgParser :: NoArgs => UsedBy :: Linker , ArgParser :: List (list) => { let Some (l) = list . single () else { cx . expected_single_argument (list . span) ; return ; } ; match l . meta_item () . and_then (| i | i . path () . word_sym ()) { Some (sym :: compiler) => { if ! cx . features () . used_with_arg () { feature_err (& cx . sess () , sym :: used_with_arg , cx . attr_span , "`#[used(compiler)]` is currently unstable" ,) . emit () ; } UsedBy :: Compiler } Some (sym :: linker) => { if ! cx . features () . used_with_arg () { feature_err (& cx . sess () , sym :: used_with_arg , cx . attr_span , "`#[used(linker)]` is currently unstable" ,) . emit () ; } UsedBy :: Linker } _ => { cx . expected_specific_argument (l . span () , & [sym :: compiler , sym :: linker]) ; return ; } } } ArgParser :: NameValue (_) => return , } ; let target = match used_by { UsedBy :: Compiler => & mut group . first_compiler , UsedBy :: Linker => & mut group . first_linker , } ; let attr_span = cx . attr_span ; if let Some (prev) = * target { cx . warn_unused_duplicate (prev , attr_span) ; } else { * target = Some (attr_span) ; } } ,)] ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Static) , Warn (Target :: MacroCall)]) ; fn finalize (self , _cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { Some (match (self . first_compiler , self . first_linker) { (_ , Some (span)) => AttributeKind :: Used { used_by : UsedBy :: Linker , span } , (Some (span) , _) => AttributeKind :: Used { used_by : UsedBy :: Compiler , span } , (None , None) => return None , }) } }}}

macro_rules! parse_tf_attribute_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_tf_attribute in module {}", module_path!());
    };
}

mkfn!{
    parse_tf_attribute_introspect!();
    fn parse_tf_attribute < 'c , S : Stage > (cx : & 'c mut AcceptContext < '_ , '_ , S > , args : & 'c ArgParser < '_ > ,) -> impl IntoIterator < Item = (Symbol , Span) > + 'c { let mut features = Vec :: new () ; let ArgParser :: List (list) = args else { cx . expected_list (cx . attr_span) ; return features ; } ; if list . is_empty () { cx . warn_empty_attribute (cx . attr_span) ; return features ; } for item in list . mixed () { let Some (name_value) = item . meta_item () else { cx . expected_name_value (item . span () , Some (sym :: enable)) ; return features ; } ; let Some (name) = name_value . path () . word_sym () else { cx . expected_name_value (name_value . path () . span () , Some (sym :: enable)) ; return features ; } ; if name != sym :: enable { cx . expected_name_value (name_value . path () . span () , Some (sym :: enable)) ; return features ; } let Some (name_value) = name_value . args () . name_value () else { cx . expected_name_value (item . span () , Some (sym :: enable)) ; return features ; } ; let Some (value_str) = name_value . value_as_str () else { cx . expected_string_literal (name_value . value_span , Some (name_value . value_as_lit ())) ; return features ; } ; for feature in value_str . as_str () . split (",") { features . push ((Symbol :: intern (feature) , item . span ())) ; } } features }
}
mkitem!{mkstruct!{pub (crate) struct TargetFeatureParser ;}}
mkitem!{mkimpl!{impl < S : Stage > CombineAttributeParser < S > for TargetFeatureParser { type Item = (Symbol , Span) ; const PATH : & [Symbol] = & [sym :: target_feature] ; const CONVERT : ConvertFn < Self :: Item > = | items , span | AttributeKind :: TargetFeature { features : items , attr_span : span , was_forced : false , } ; const TEMPLATE : AttributeTemplate = template ! (List : & ["enable = \"feat1, feat2\""]) ; fn extend < 'c > (cx : & 'c mut AcceptContext < '_ , '_ , S > , args : & 'c ArgParser < '_ > ,) -> impl IntoIterator < Item = Self :: Item > + 'c { parse_tf_attribute (cx , args) } const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) , Warn (Target :: Statement) , Warn (Target :: Field) , Warn (Target :: Arm) , Warn (Target :: MacroDef) , Warn (Target :: MacroCall) ,]) ; }}}
mkitem!{mkstruct!{pub (crate) struct ForceTargetFeatureParser ;}}
mkitem!{mkimpl!{impl < S : Stage > CombineAttributeParser < S > for ForceTargetFeatureParser { type Item = (Symbol , Span) ; const PATH : & [Symbol] = & [sym :: force_target_feature] ; const CONVERT : ConvertFn < Self :: Item > = | items , span | AttributeKind :: TargetFeature { features : items , attr_span : span , was_forced : true , } ; const TEMPLATE : AttributeTemplate = template ! (List : & ["enable = \"feat1, feat2\""]) ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) ,]) ; fn extend < 'c > (cx : & 'c mut AcceptContext < '_ , '_ , S > , args : & 'c ArgParser < '_ > ,) -> impl IntoIterator < Item = Self :: Item > + 'c { parse_tf_attribute (cx , args) } }}}
mkitem!{mkstruct!{pub (crate) struct SanitizeParser ;}}
mkitem!{mkimpl!{impl < S : Stage > SingleAttributeParser < S > for SanitizeParser { const PATH : & [Symbol] = & [sym :: sanitize] ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; const TEMPLATE : AttributeTemplate = template ! (List : & [r#"address = "on|off""# , r#"kernel_address = "on|off""# , r#"cfi = "on|off""# , r#"hwaddress = "on|off""# , r#"kcfi = "on|off""# , r#"memory = "on|off""# , r#"memtag = "on|off""# , r#"shadow_call_stack = "on|off""# , r#"thread = "on|off""#]) ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let Some (list) = args . list () else { cx . expected_list (cx . attr_span) ; return None ; } ; let mut on_set = SanitizerSet :: empty () ; let mut off_set = SanitizerSet :: empty () ; for item in list . mixed () { let Some (item) = item . meta_item () else { cx . expected_name_value (item . span () , None) ; continue ; } ; let path = item . path () . word_sym () ; let Some (value) = item . args () . name_value () else { cx . expected_name_value (item . span () , path) ; continue ; } ; let mut apply = | s : SanitizerSet | { let is_on = match value . value_as_str () { Some (sym :: on) => true , Some (sym :: off) => false , Some (_) => { cx . expected_specific_argument_strings (value . value_span , & [sym :: on , sym :: off] ,) ; return ; } None => { cx . expected_string_literal (value . value_span , Some (value . value_as_lit ())) ; return ; } } ; if is_on { on_set |= s ; } else { off_set |= s ; } } ; match path { Some (sym :: address) | Some (sym :: kernel_address) => { apply (SanitizerSet :: ADDRESS | SanitizerSet :: KERNELADDRESS) } Some (sym :: cfi) => apply (SanitizerSet :: CFI) , Some (sym :: kcfi) => apply (SanitizerSet :: KCFI) , Some (sym :: memory) => apply (SanitizerSet :: MEMORY) , Some (sym :: memtag) => apply (SanitizerSet :: MEMTAG) , Some (sym :: shadow_call_stack) => apply (SanitizerSet :: SHADOWCALLSTACK) , Some (sym :: thread) => apply (SanitizerSet :: THREAD) , Some (sym :: hwaddress) => apply (SanitizerSet :: HWADDRESS) , _ => { cx . expected_specific_argument_strings (item . path () . span () , & [sym :: address , sym :: cfi , sym :: kcfi , sym :: memory , sym :: memtag , sym :: shadow_call_stack , sym :: thread , sym :: hwaddress ,] ,) ; continue ; } } } Some (AttributeKind :: Sanitize { on_set , off_set , span : cx . attr_span }) } }}}