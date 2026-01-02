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
mkuse!{use rustc_ast :: { LitKind , NodeId } ;}
mkuse!{use rustc_feature :: { AttributeTemplate , Features , template } ;}
mkuse!{use rustc_hir :: RustcVersion ;}
mkuse!{use rustc_hir :: attrs :: CfgEntry ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: config :: ExpectedValues ;}
mkuse!{use rustc_session :: lint :: BuiltinLintDiag ;}
mkuse!{use rustc_session :: lint :: builtin :: UNEXPECTED_CFGS ;}
mkuse!{use rustc_session :: parse :: feature_err ;}
mkuse!{use rustc_span :: { Span , Symbol , sym } ;}
mkuse!{use thin_vec :: ThinVec ;}
mkuse!{use crate :: context :: { AcceptContext , ShouldEmit , Stage } ;}
mkuse!{use crate :: parser :: { ArgParser , MetaItemListParser , MetaItemOrLitParser , NameValueParser } ;}
mkuse!{use crate :: { CfgMatchesLintEmitter , fluent_generated , parse_version , session_diagnostics , try_gate_cfg , } ;}
mkitem!{pub const CFG_TEMPLATE : AttributeTemplate = template ! (List : & ["predicate"] , "https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute") ;}

macro_rules! parse_cfg_attr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_cfg_attr in module {}", module_path!());
    };
}

mkfn!{
    parse_cfg_attr_introspect!();
    pub fn parse_cfg_attr < 'c , S : Stage > (cx : & 'c mut AcceptContext < '_ , '_ , S > , args : & 'c ArgParser < '_ > ,) -> Option < CfgEntry > { let ArgParser :: List (list) = args else { cx . expected_list (cx . attr_span) ; return None ; } ; let Some (single) = list . single () else { cx . expected_single_argument (list . span) ; return None ; } ; parse_cfg_entry (cx , single) }
}

macro_rules! parse_cfg_entry_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_cfg_entry in module {}", module_path!());
    };
}

mkfn!{
    parse_cfg_entry_introspect!();
    pub (crate) fn parse_cfg_entry < S : Stage > (cx : & mut AcceptContext < '_ , '_ , S > , item : & MetaItemOrLitParser < '_ > ,) -> Option < CfgEntry > { Some (match item { MetaItemOrLitParser :: MetaItemParser (meta) => match meta . args () { ArgParser :: List (list) => match meta . path () . word_sym () { Some (sym :: not) => { let Some (single) = list . single () else { cx . expected_single_argument (list . span) ; return None ; } ; CfgEntry :: Not (Box :: new (parse_cfg_entry (cx , single) ?) , list . span) } Some (sym :: any) => CfgEntry :: Any (list . mixed () . flat_map (| sub_item | parse_cfg_entry (cx , sub_item)) . collect () , list . span ,) , Some (sym :: all) => CfgEntry :: All (list . mixed () . flat_map (| sub_item | parse_cfg_entry (cx , sub_item)) . collect () , list . span ,) , Some (sym :: target) => parse_cfg_entry_target (cx , list , meta . span ()) ? , Some (sym :: version) => parse_cfg_entry_version (cx , list , meta . span ()) ? , _ => { cx . emit_err (session_diagnostics :: InvalidPredicate { span : meta . span () , predicate : meta . path () . to_string () , }) ; return None ; } } , a @ (ArgParser :: NoArgs | ArgParser :: NameValue (_)) => { let Some (name) = meta . path () . word_sym () else { cx . emit_err (session_diagnostics :: CfgPredicateIdentifier { span : meta . path () . span () , }) ; return None ; } ; parse_name_value (name , meta . path () . span () , a . name_value () , meta . span () , cx) ? } } , MetaItemOrLitParser :: Lit (lit) => match lit . kind { LitKind :: Bool (b) => CfgEntry :: Bool (b , lit . span) , _ => { cx . emit_err (session_diagnostics :: CfgPredicateIdentifier { span : lit . span }) ; return None ; } } , MetaItemOrLitParser :: Err (_ , _) => return None , }) }
}

macro_rules! parse_cfg_entry_version_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_cfg_entry_version in module {}", module_path!());
    };
}

mkfn!{
    parse_cfg_entry_version_introspect!();
    fn parse_cfg_entry_version < S : Stage > (cx : & mut AcceptContext < '_ , '_ , S > , list : & MetaItemListParser < '_ > , meta_span : Span ,) -> Option < CfgEntry > { try_gate_cfg (sym :: version , meta_span , cx . sess () , cx . features_option ()) ; let Some (version) = list . single () else { cx . emit_err (session_diagnostics :: ExpectedSingleVersionLiteral { span : list . span }) ; return None ; } ; let Some (version_lit) = version . lit () else { cx . emit_err (session_diagnostics :: ExpectedVersionLiteral { span : version . span () }) ; return None ; } ; let Some (version_str) = version_lit . value_str () else { cx . emit_err (session_diagnostics :: ExpectedVersionLiteral { span : version_lit . span }) ; return None ; } ; let min_version = parse_version (version_str) . or_else (| | { cx . sess () . dcx () . emit_warn (session_diagnostics :: UnknownVersionLiteral { span : version_lit . span }) ; None }) ; Some (CfgEntry :: Version (min_version , list . span)) }
}

macro_rules! parse_cfg_entry_target_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_cfg_entry_target in module {}", module_path!());
    };
}

mkfn!{
    parse_cfg_entry_target_introspect!();
    fn parse_cfg_entry_target < S : Stage > (cx : & mut AcceptContext < '_ , '_ , S > , list : & MetaItemListParser < '_ > , meta_span : Span ,) -> Option < CfgEntry > { if let Some (features) = cx . features_option () && ! features . cfg_target_compact () { feature_err (cx . sess () , sym :: cfg_target_compact , meta_span , fluent_generated :: attr_parsing_unstable_cfg_target_compact ,) . emit () ; } let mut result = ThinVec :: new () ; for sub_item in list . mixed () { let Some (sub_item) = sub_item . meta_item () else { cx . expected_name_value (sub_item . span () , None) ; continue ; } ; let Some (nv) = sub_item . args () . name_value () else { cx . expected_name_value (sub_item . span () , None) ; continue ; } ; let Some (name) = sub_item . path () . word_sym () else { cx . emit_err (session_diagnostics :: CfgPredicateIdentifier { span : sub_item . path () . span () , }) ; return None ; } ; let name = Symbol :: intern (& format ! ("target_{name}")) ; if let Some (cfg) = parse_name_value (name , sub_item . path () . span () , Some (nv) , sub_item . span () , cx) { result . push (cfg) ; } } Some (CfgEntry :: All (result , list . span)) }
}

macro_rules! parse_name_value_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_name_value in module {}", module_path!());
    };
}

mkfn!{
    parse_name_value_introspect!();
    fn parse_name_value < S : Stage > (name : Symbol , name_span : Span , value : Option < & NameValueParser > , span : Span , cx : & mut AcceptContext < '_ , '_ , S > ,) -> Option < CfgEntry > { try_gate_cfg (name , span , cx . sess () , cx . features_option ()) ; let value = match value { None => None , Some (value) => { let Some (value_str) = value . value_as_str () else { cx . expected_string_literal (value . value_span , Some (value . value_as_lit ())) ; return None ; } ; Some ((value_str , value . value_span)) } } ; Some (CfgEntry :: NameValue { name , name_span , value , span }) }
}

macro_rules! eval_config_entry_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function eval_config_entry in module {}", module_path!());
    };
}

mkfn!{
    eval_config_entry_introspect!();
    pub fn eval_config_entry (sess : & Session , cfg_entry : & CfgEntry , id : NodeId , features : Option < & Features > , emit_lints : ShouldEmit ,) -> EvalConfigResult { match cfg_entry { CfgEntry :: All (subs , ..) => { let mut all = None ; for sub in subs { let res = eval_config_entry (sess , sub , id , features , emit_lints) ; if ! res . as_bool () { all . get_or_insert (res) ; } } all . unwrap_or_else (| | EvalConfigResult :: True) } CfgEntry :: Any (subs , span) => { let mut any = None ; for sub in subs { let res = eval_config_entry (sess , sub , id , features , emit_lints) ; if res . as_bool () { any . get_or_insert (res) ; } } any . unwrap_or_else (| | EvalConfigResult :: False { reason : cfg_entry . clone () , reason_span : * span , }) } CfgEntry :: Not (sub , span) => { if eval_config_entry (sess , sub , id , features , emit_lints) . as_bool () { EvalConfigResult :: False { reason : cfg_entry . clone () , reason_span : * span } } else { EvalConfigResult :: True } } CfgEntry :: Bool (b , span) => { if * b { EvalConfigResult :: True } else { EvalConfigResult :: False { reason : cfg_entry . clone () , reason_span : * span } } } CfgEntry :: NameValue { name , name_span , value , span } => { if let ShouldEmit :: ErrorsAndLints = emit_lints { match sess . psess . check_config . expecteds . get (name) { Some (ExpectedValues :: Some (values)) if ! values . contains (& value . map (| (v , _) | v)) => { id . emit_span_lint (sess , UNEXPECTED_CFGS , * span , BuiltinLintDiag :: UnexpectedCfgValue ((* name , * name_span) , * value) ,) ; } None if sess . psess . check_config . exhaustive_names => { id . emit_span_lint (sess , UNEXPECTED_CFGS , * span , BuiltinLintDiag :: UnexpectedCfgName ((* name , * name_span) , * value) ,) ; } _ => { } } } if sess . psess . config . contains (& (* name , value . map (| (v , _) | v))) { EvalConfigResult :: True } else { EvalConfigResult :: False { reason : cfg_entry . clone () , reason_span : * span } } } CfgEntry :: Version (min_version , version_span) => { let Some (min_version) = min_version else { return EvalConfigResult :: False { reason : cfg_entry . clone () , reason_span : * version_span , } ; } ; let min_version_ok = if sess . psess . assume_incomplete_release { RustcVersion :: current_overridable () > * min_version } else { RustcVersion :: current_overridable () >= * min_version } ; if min_version_ok { EvalConfigResult :: True } else { EvalConfigResult :: False { reason : cfg_entry . clone () , reason_span : * version_span } } } } }
}
mkitem!{mkenum!{pub enum EvalConfigResult { True , False { reason : CfgEntry , reason_span : Span } , }}}
mkitem!{mkimpl!{impl EvalConfigResult { pub fn as_bool (& self) -> bool { match self { EvalConfigResult :: True => true , EvalConfigResult :: False { .. } => false , } } }}}