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
mkuse!{use rustc_abi :: ExternAbi ;}
mkuse!{use rustc_attr_parsing :: AttributeParser ;}
mkuse!{use rustc_errors :: Applicability ;}
mkuse!{use rustc_hir :: attrs :: { AttributeKind , ReprAttr } ;}
mkuse!{use rustc_hir :: def :: { DefKind , Res } ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_hir :: intravisit :: { FnKind , Visitor } ;}
mkuse!{use rustc_hir :: { Attribute , GenericParamKind , PatExprKind , PatKind , find_attr } ;}
mkuse!{use rustc_middle :: hir :: nested_filter :: All ;}
mkuse!{use rustc_middle :: ty :: AssocContainer ;}
mkuse!{use rustc_session :: config :: CrateType ;}
mkuse!{use rustc_session :: { declare_lint , declare_lint_pass } ;}
mkuse!{use rustc_span :: def_id :: LocalDefId ;}
mkuse!{use rustc_span :: { BytePos , Ident , Span , sym } ;}
mkuse!{use { rustc_ast as ast , rustc_hir as hir } ;}
mkuse!{use crate :: lints :: { NonCamelCaseType , NonCamelCaseTypeSub , NonSnakeCaseDiag , NonSnakeCaseDiagSub , NonUpperCaseGlobal , NonUpperCaseGlobalSub , NonUpperCaseGlobalSubTool , } ;}
mkuse!{use crate :: { EarlyContext , EarlyLintPass , LateContext , LateLintPass , LintContext } ;}
mkitem!{declare_lint ! { # [doc = " The `non_camel_case_types` lint detects types, variants, traits and"] # [doc = " type parameters that don't have camel case names."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " struct my_struct;"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " The preferred style for these identifiers is to use \"camel case\", such"] # [doc = " as `MyStruct`, where the first letter should not be lowercase, and"] # [doc = " should not use underscores between letters. Underscores are allowed at"] # [doc = " the beginning and end of the identifier, as well as between"] # [doc = " non-letters (such as `X86_64`)."] pub NON_CAMEL_CASE_TYPES , Warn , "types, variants, traits and type parameters should have camel case names" }}
mkitem!{declare_lint_pass ! (NonCamelCaseTypes => [NON_CAMEL_CASE_TYPES]) ;}

macro_rules! char_has_case_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function char_has_case in module {}", module_path!());
    };
}

mkfn!{
    char_has_case_introspect!();
    # [doc = " Some unicode characters *have* case, are considered upper case or lower case, but they *can't*"] # [doc = " be upper cased or lower cased. For the purposes of the lint suggestion, we care about being able"] # [doc = " to change the char's case."] fn char_has_case (c : char) -> bool { let mut l = c . to_lowercase () ; let mut u = c . to_uppercase () ; while let Some (l) = l . next () { match u . next () { Some (u) if l != u => return true , _ => { } } } u . next () . is_some () }
}

macro_rules! is_camel_case_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_camel_case in module {}", module_path!());
    };
}

mkfn!{
    is_camel_case_introspect!();
    fn is_camel_case (name : & str) -> bool { let name = name . trim_matches ('_') ; if name . is_empty () { return true ; } ! name . chars () . next () . unwrap () . is_lowercase () && ! name . contains ("__") && ! name . chars () . collect :: < Vec < _ > > () . array_windows () . any (| & [fst , snd] | { char_has_case (fst) && snd == '_' || char_has_case (snd) && fst == '_' }) }
}

macro_rules! to_camel_case_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_camel_case in module {}", module_path!());
    };
}

mkfn!{
    to_camel_case_introspect!();
    fn to_camel_case (s : & str) -> String { s . trim_matches ('_') . split ('_') . filter (| component | ! component . is_empty ()) . map (| component | { let mut camel_cased_component = String :: new () ; let mut new_word = true ; let mut prev_is_lower_case = true ; for c in component . chars () { if prev_is_lower_case && c . is_uppercase () { new_word = true ; } if new_word { camel_cased_component . extend (c . to_uppercase ()) ; } else { camel_cased_component . extend (c . to_lowercase ()) ; } prev_is_lower_case = c . is_lowercase () ; new_word = false ; } camel_cased_component }) . fold ((String :: new () , None) , | (acc , prev) : (String , Option < String >) , next | { let join = if let Some (prev) = prev { let l = prev . chars () . last () . unwrap () ; let f = next . chars () . next () . unwrap () ; ! char_has_case (l) && ! char_has_case (f) } else { false } ; (acc + if join { "_" } else { "" } + & next , Some (next)) }) . 0 }
}
mkitem!{mkimpl!{impl NonCamelCaseTypes { fn check_case (& self , cx : & EarlyContext < '_ > , sort : & str , ident : & Ident) { let name = ident . name . as_str () ; if ! is_camel_case (name) { let cc = to_camel_case (name) ; let sub = if * name != cc { NonCamelCaseTypeSub :: Suggestion { span : ident . span , replace : cc } } else { NonCamelCaseTypeSub :: Label { span : ident . span } } ; cx . emit_span_lint (NON_CAMEL_CASE_TYPES , ident . span , NonCamelCaseType { sort , name , sub } ,) ; } } }}}
mkitem!{mkimpl!{impl EarlyLintPass for NonCamelCaseTypes { fn check_item (& mut self , cx : & EarlyContext < '_ > , it : & ast :: Item) { let has_repr_c = matches ! (AttributeParser :: parse_limited (cx . sess () , & it . attrs , sym :: repr , it . span , it . id , None) , Some (Attribute :: Parsed (AttributeKind :: Repr { reprs , .. })) if reprs . iter () . any (| (r , _) | r == & ReprAttr :: ReprC)) ; if has_repr_c { return ; } match & it . kind { ast :: ItemKind :: TyAlias (box ast :: TyAlias { ident , .. }) | ast :: ItemKind :: Enum (ident , ..) | ast :: ItemKind :: Struct (ident , ..) | ast :: ItemKind :: Union (ident , ..) => self . check_case (cx , "type" , ident) , ast :: ItemKind :: Trait (box ast :: Trait { ident , .. }) => { self . check_case (cx , "trait" , ident) } ast :: ItemKind :: TraitAlias (ident , _ , _) => self . check_case (cx , "trait alias" , ident) , ast :: ItemKind :: Impl (ast :: Impl { of_trait : None , items , .. }) => { for it in items { if let ast :: AssocItemKind :: Type (alias) = & it . kind { self . check_case (cx , "associated type" , & alias . ident) ; } } } _ => () , } } fn check_trait_item (& mut self , cx : & EarlyContext < '_ > , it : & ast :: AssocItem) { if let ast :: AssocItemKind :: Type (alias) = & it . kind { self . check_case (cx , "associated type" , & alias . ident) ; } } fn check_variant (& mut self , cx : & EarlyContext < '_ > , v : & ast :: Variant) { self . check_case (cx , "variant" , & v . ident) ; } fn check_generic_param (& mut self , cx : & EarlyContext < '_ > , param : & ast :: GenericParam) { if let ast :: GenericParamKind :: Type { .. } = param . kind { self . check_case (cx , "type parameter" , & param . ident) ; } } }}}
mkitem!{declare_lint ! { # [doc = " The `non_snake_case` lint detects variables, methods, functions,"] # [doc = " lifetime parameters and modules that don't have snake case names."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " let MY_VALUE = 5;"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " The preferred style for these identifiers is to use \"snake case\","] # [doc = " where all the characters are in lowercase, with words separated with a"] # [doc = " single underscore, such as `my_value`."] pub NON_SNAKE_CASE , Warn , "variables, methods, functions, lifetime parameters and modules should have snake case names" }}
mkitem!{declare_lint_pass ! (NonSnakeCase => [NON_SNAKE_CASE]) ;}
mkitem!{mkimpl!{impl NonSnakeCase { fn to_snake_case (mut name : & str) -> String { let mut words = vec ! [] ; name = name . trim_start_matches (| c : char | { if c == '_' { words . push (String :: new ()) ; true } else { false } }) ; for s in name . split ('_') { let mut last_upper = false ; let mut buf = String :: new () ; if s . is_empty () { continue ; } for ch in s . chars () { if ! buf . is_empty () && buf != "'" && ch . is_uppercase () && ! last_upper { words . push (buf) ; buf = String :: new () ; } last_upper = ch . is_uppercase () ; buf . extend (ch . to_lowercase ()) ; } words . push (buf) ; } words . join ("_") } # [doc = " Checks if a given identifier is snake case, and reports a diagnostic if not."] fn check_snake_case (& self , cx : & LateContext < '_ > , sort : & str , ident : & Ident) { fn is_snake_case (ident : & str) -> bool { if ident . is_empty () { return true ; } let ident = ident . trim_start_matches ('\'') ; let ident = ident . trim_matches ('_') ; if ident . contains ("__") { return false ; } ! ident . chars () . any (char :: is_uppercase) } let name = ident . name . as_str () ; if ! is_snake_case (name) { let span = ident . span ; let sc = NonSnakeCase :: to_snake_case (name) ; let sub = if name != sc { if ! span . is_dummy () { let sc_ident = Ident :: from_str_and_span (& sc , span) ; if sc_ident . is_reserved () { if sc_ident . name . can_be_raw () { NonSnakeCaseDiagSub :: RenameOrConvertSuggestion { span , suggestion : sc_ident , } } else { NonSnakeCaseDiagSub :: SuggestionAndNote { span } } } else { NonSnakeCaseDiagSub :: ConvertSuggestion { span , suggestion : sc . clone () } } } else { NonSnakeCaseDiagSub :: Help } } else { NonSnakeCaseDiagSub :: Label { span } } ; cx . emit_span_lint (NON_SNAKE_CASE , span , NonSnakeCaseDiag { sort , name , sc , sub }) ; } } }}}
mkitem!{mkimpl!{impl < 'tcx > LateLintPass < 'tcx > for NonSnakeCase { fn check_mod (& mut self , cx : & LateContext < '_ > , _ : & 'tcx hir :: Mod < 'tcx > , id : hir :: HirId) { if id != hir :: CRATE_HIR_ID { return ; } if cx . tcx . crate_types () . iter () . all (| & crate_type | crate_type == CrateType :: Executable) { return ; } let crate_ident = if let Some (name) = & cx . tcx . sess . opts . crate_name { Some (Ident :: from_str (name)) } else { find_attr ! (cx . tcx . hir_attrs (hir :: CRATE_HIR_ID) , AttributeKind :: CrateName { name , name_span ,.. } => (name , name_span)) . map (| (& name , & span) | { let sp = cx . sess () . source_map () . span_to_snippet (span) . ok () . and_then (| snippet | { let left = snippet . find ('"') ? ; let right = snippet . rfind ('"') . map (| pos | snippet . len () - pos) ? ; Some (span . with_lo (span . lo () + BytePos (left as u32 + 1)) . with_hi (span . hi () - BytePos (right as u32)) ,) }) . unwrap_or (span) ; Ident :: new (name , sp) } ,) } ; if let Some (ident) = & crate_ident { self . check_snake_case (cx , "crate" , ident) ; } } fn check_generic_param (& mut self , cx : & LateContext < '_ > , param : & hir :: GenericParam < '_ >) { if let GenericParamKind :: Lifetime { .. } = param . kind { self . check_snake_case (cx , "lifetime" , & param . name . ident ()) ; } } fn check_fn (& mut self , cx : & LateContext < '_ > , fk : FnKind < '_ > , _ : & hir :: FnDecl < '_ > , _ : & hir :: Body < '_ > , _ : Span , id : LocalDefId ,) { match & fk { FnKind :: Method (ident , sig , ..) => match cx . tcx . associated_item (id) . container { AssocContainer :: InherentImpl => { if sig . header . abi != ExternAbi :: Rust && find_attr ! (cx . tcx . get_all_attrs (id) , AttributeKind :: NoMangle (..)) { return ; } self . check_snake_case (cx , "method" , ident) ; } AssocContainer :: Trait => { self . check_snake_case (cx , "trait method" , ident) ; } AssocContainer :: TraitImpl (_) => { } } , FnKind :: ItemFn (ident , _ , header) => { if header . abi != ExternAbi :: Rust && find_attr ! (cx . tcx . get_all_attrs (id) , AttributeKind :: NoMangle (..)) { return ; } self . check_snake_case (cx , "function" , ident) ; } FnKind :: Closure => () , } } fn check_item (& mut self , cx : & LateContext < '_ > , it : & hir :: Item < '_ >) { if let hir :: ItemKind :: Mod (ident , _) = it . kind { self . check_snake_case (cx , "module" , & ident) ; } } fn check_ty (& mut self , cx : & LateContext < '_ > , ty : & hir :: Ty < '_ , hir :: AmbigArg >) { if let hir :: TyKind :: FnPtr (hir :: FnPtrTy { param_idents , .. }) = & ty . kind { for param_ident in * param_idents { if let Some (param_ident) = param_ident { self . check_snake_case (cx , "variable" , param_ident) ; } } } } fn check_trait_item (& mut self , cx : & LateContext < '_ > , item : & hir :: TraitItem < '_ >) { if let hir :: TraitItemKind :: Fn (_ , hir :: TraitFn :: Required (param_idents)) = item . kind { self . check_snake_case (cx , "trait method" , & item . ident) ; for param_ident in param_idents { if let Some (param_ident) = param_ident { self . check_snake_case (cx , "variable" , param_ident) ; } } } } fn check_pat (& mut self , cx : & LateContext < '_ > , p : & hir :: Pat < '_ >) { if let PatKind :: Binding (_ , hid , ident , _) = p . kind { if let hir :: Node :: PatField (field) = cx . tcx . parent_hir_node (hid) { if ! field . is_shorthand { self . check_snake_case (cx , "variable" , & ident) ; } return ; } self . check_snake_case (cx , "variable" , & ident) ; } } fn check_struct_def (& mut self , cx : & LateContext < '_ > , s : & hir :: VariantData < '_ >) { for sf in s . fields () { self . check_snake_case (cx , "structure field" , & sf . ident) ; } } }}}
mkitem!{declare_lint ! { # [doc = " The `non_upper_case_globals` lint detects static items that don't have"] # [doc = " uppercase identifiers."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " static max_points: i32 = 5;"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " The preferred style is for static item names to use all uppercase"] # [doc = " letters such as `MAX_POINTS`."] pub NON_UPPER_CASE_GLOBALS , Warn , "static constants should have uppercase identifiers" }}
mkitem!{declare_lint_pass ! (NonUpperCaseGlobals => [NON_UPPER_CASE_GLOBALS]) ;}
mkitem!{mkimpl!{impl NonUpperCaseGlobals { fn check_upper_case (cx : & LateContext < '_ > , sort : & str , did : Option < LocalDefId > , ident : & Ident) { let name = ident . name . as_str () ; if name . chars () . any (| c | c . is_lowercase ()) { let uc = NonSnakeCase :: to_snake_case (name) . to_uppercase () ; let can_change_usages = if let Some (did) = did { ! cx . tcx . effective_visibilities (()) . is_exported (did) } else { false } ; let sub = if * name != uc { NonUpperCaseGlobalSub :: Suggestion { span : ident . span , replace : uc . clone () , applicability : if can_change_usages { Applicability :: MachineApplicable } else { Applicability :: MaybeIncorrect } , } } else { NonUpperCaseGlobalSub :: Label { span : ident . span } } ; struct UsageCollector < 'a , 'tcx > { cx : & 'tcx LateContext < 'a > , did : DefId , collected : Vec < Span > , } impl < 'v , 'tcx > Visitor < 'v > for UsageCollector < 'v , 'tcx > { type NestedFilter = All ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } fn visit_path (& mut self , path : & rustc_hir :: Path < 'v > , _id : rustc_hir :: HirId ,) -> Self :: Result { if let Some (final_seg) = path . segments . last () && final_seg . res . opt_def_id () == Some (self . did) { self . collected . push (final_seg . ident . span) ; } } } cx . emit_span_lint_lazy (NON_UPPER_CASE_GLOBALS , ident . span , | | { let usages = if can_change_usages && * name != uc && let Some (did) = did { let mut usage_collector = UsageCollector { cx , did : did . to_def_id () , collected : Vec :: new () } ; cx . tcx . hir_walk_toplevel_module (& mut usage_collector) ; usage_collector . collected . into_iter () . map (| span | NonUpperCaseGlobalSubTool { span , replace : uc . clone () }) . collect () } else { vec ! [] } ; NonUpperCaseGlobal { sort , name , sub , usages } }) ; } } }}}
mkitem!{mkimpl!{impl < 'tcx > LateLintPass < 'tcx > for NonUpperCaseGlobals { fn check_item (& mut self , cx : & LateContext < '_ > , it : & hir :: Item < '_ >) { let attrs = cx . tcx . hir_attrs (it . hir_id ()) ; match it . kind { hir :: ItemKind :: Static (_ , ident , ..) if ! find_attr ! (attrs , AttributeKind :: NoMangle (..)) => { NonUpperCaseGlobals :: check_upper_case (cx , "static variable" , Some (it . owner_id . def_id) , & ident ,) ; } hir :: ItemKind :: Const (ident , ..) => { NonUpperCaseGlobals :: check_upper_case (cx , "constant" , Some (it . owner_id . def_id) , & ident ,) ; } _ => { } } } fn check_trait_item (& mut self , cx : & LateContext < '_ > , ti : & hir :: TraitItem < '_ >) { if let hir :: TraitItemKind :: Const (..) = ti . kind { NonUpperCaseGlobals :: check_upper_case (cx , "associated constant" , None , & ti . ident) ; } } fn check_impl_item (& mut self , cx : & LateContext < '_ > , ii : & hir :: ImplItem < '_ >) { if let hir :: ImplItemKind :: Const (..) = ii . kind && let hir :: ImplItemImplKind :: Inherent { .. } = ii . impl_kind { NonUpperCaseGlobals :: check_upper_case (cx , "associated constant" , None , & ii . ident) ; } } fn check_pat (& mut self , cx : & LateContext < '_ > , p : & hir :: Pat < '_ >) { if let PatKind :: Expr (hir :: PatExpr { kind : PatExprKind :: Path (hir :: QPath :: Resolved (None , path)) , .. }) = p . kind { if let Res :: Def (DefKind :: Const , _) = path . res && let [segment] = path . segments { NonUpperCaseGlobals :: check_upper_case (cx , "constant in pattern" , None , & segment . ident ,) ; } } } fn check_generic_param (& mut self , cx : & LateContext < '_ > , param : & hir :: GenericParam < '_ >) { if let GenericParamKind :: Const { .. } = param . kind { NonUpperCaseGlobals :: check_upper_case (cx , "const parameter" , Some (param . def_id) , & param . name . ident () ,) ; } } }}}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}