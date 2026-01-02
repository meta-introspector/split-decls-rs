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
mkuse!{use rustc_ast :: token :: TokenKind ;}
mkuse!{use rustc_span :: symbol :: { Symbol , kw , sym } ;}
mkitem!{mkenum!{# [doc = " Used in \"expected\"/\"expected one of\" error messages. Tokens are added here"] # [doc = " as necessary. Tokens with values (e.g. literals, identifiers) are"] # [doc = " represented by a single variant (e.g. `Literal`, `Ident`)."] # [doc = ""] # [doc = " It's an awkward representation, but it's important for performance. It's a"] # [doc = " C-style parameterless enum so that `TokenTypeSet` can be a bitset. This is"] # [doc = " important because `Parser::expected_token_types` is very hot. `TokenType`"] # [doc = " used to have variants with parameters (e.g. all the keywords were in a"] # [doc = " single `Keyword` variant with a `Symbol` parameter) and"] # [doc = " `Parser::expected_token_types` was a `Vec<TokenType>` which was much slower"] # [doc = " to manipulate."] # [doc = ""] # [doc = " We really want to keep the number of variants to 128 or fewer, so that"] # [doc = " `TokenTypeSet` can be implemented with a `u128`."] # [derive (Debug , Clone , Copy , PartialEq)] pub enum TokenType { Eq , Lt , Le , EqEq , Gt , AndAnd , OrOr , Bang , Tilde , Plus , Minus , Star , And , Or , At , Dot , DotDot , DotDotDot , DotDotEq , Comma , Semi , Colon , PathSep , RArrow , FatArrow , Pound , Question , OpenParen , CloseParen , OpenBrace , CloseBrace , OpenBracket , CloseBracket , Eof , # [doc = " Any operator."] Operator , # [doc = " Any identifier token."] Ident , # [doc = " Any lifetime token."] Lifetime , # [doc = " Any token that can start a path."] Path , # [doc = " Any token that can start a type."] Type , # [doc = " Any token that can start a const expression."] Const , KwAs , KwAsync , KwAuto , KwAwait , KwBecome , KwBox , KwBreak , KwCatch , KwConst , KwContinue , KwContractEnsures , KwContractRequires , KwCrate , KwDefault , KwDyn , KwElse , KwEnum , KwExtern , KwFn , KwFor , KwGen , KwIf , KwImpl , KwIn , KwLet , KwLoop , KwMacro , KwMacroRules , KwMatch , KwMod , KwMove , KwMut , KwPub , KwRaw , KwRef , KwReturn , KwReuse , KwSafe , KwSelfUpper , KwStatic , KwStruct , KwSuper , KwTrait , KwTry , KwType , KwUnderscore , KwUnsafe , KwUse , KwWhere , KwWhile , KwYield , SymAttSyntax , SymClobberAbi , SymInlateout , SymInout , SymIs , SymLabel , SymLateout , SymMayUnwind , SymNomem , SymNoreturn , SymNostack , SymOptions , SymOut , SymPreservesFlags , SymPure , SymReadonly , SymSym , }}}
mkitem!{macro_rules ! from_u32_match { ($ val : ident ; $ ($ tok : ident ,) +) => { match $ val { $ (t if t == TokenType ::$ tok as u32 => TokenType ::$ tok ,) + _ => panic ! ("unhandled value: {}" , $ val) , } } ; }}
mkitem!{mkimpl!{impl TokenType { fn from_u32 (val : u32) -> TokenType { let token_type = from_u32_match ! { val ; Eq , Lt , Le , EqEq , Gt , AndAnd , OrOr , Bang , Tilde , Plus , Minus , Star , And , Or , At , Dot , DotDot , DotDotDot , DotDotEq , Comma , Semi , Colon , PathSep , RArrow , FatArrow , Pound , Question , OpenParen , CloseParen , OpenBrace , CloseBrace , OpenBracket , CloseBracket , Eof , Operator , Ident , Lifetime , Path , Type , Const , KwAs , KwAsync , KwAuto , KwAwait , KwBecome , KwBox , KwBreak , KwCatch , KwConst , KwContinue , KwContractEnsures , KwContractRequires , KwCrate , KwDefault , KwDyn , KwElse , KwEnum , KwExtern , KwFn , KwFor , KwGen , KwIf , KwImpl , KwIn , KwLet , KwLoop , KwMacro , KwMacroRules , KwMatch , KwMod , KwMove , KwMut , KwPub , KwRaw , KwRef , KwReturn , KwReuse , KwSafe , KwSelfUpper , KwStatic , KwStruct , KwSuper , KwTrait , KwTry , KwType , KwUnderscore , KwUnsafe , KwUse , KwWhere , KwWhile , KwYield , SymAttSyntax , SymClobberAbi , SymInlateout , SymInout , SymIs , SymLabel , SymLateout , SymMayUnwind , SymNomem , SymNoreturn , SymNostack , SymOptions , SymOut , SymPreservesFlags , SymPure , SymReadonly , SymSym , } ; token_type } pub (super) fn is_keyword (& self) -> Option < Symbol > { match self { TokenType :: KwAs => Some (kw :: As) , TokenType :: KwAsync => Some (kw :: Async) , TokenType :: KwAuto => Some (kw :: Auto) , TokenType :: KwAwait => Some (kw :: Await) , TokenType :: KwBecome => Some (kw :: Become) , TokenType :: KwBox => Some (kw :: Box) , TokenType :: KwBreak => Some (kw :: Break) , TokenType :: KwCatch => Some (kw :: Catch) , TokenType :: KwConst => Some (kw :: Const) , TokenType :: KwContinue => Some (kw :: Continue) , TokenType :: KwContractEnsures => Some (kw :: ContractEnsures) , TokenType :: KwContractRequires => Some (kw :: ContractRequires) , TokenType :: KwCrate => Some (kw :: Crate) , TokenType :: KwDefault => Some (kw :: Default) , TokenType :: KwDyn => Some (kw :: Dyn) , TokenType :: KwElse => Some (kw :: Else) , TokenType :: KwEnum => Some (kw :: Enum) , TokenType :: KwExtern => Some (kw :: Extern) , TokenType :: KwFn => Some (kw :: Fn) , TokenType :: KwFor => Some (kw :: For) , TokenType :: KwGen => Some (kw :: Gen) , TokenType :: KwIf => Some (kw :: If) , TokenType :: KwImpl => Some (kw :: Impl) , TokenType :: KwIn => Some (kw :: In) , TokenType :: KwLet => Some (kw :: Let) , TokenType :: KwLoop => Some (kw :: Loop) , TokenType :: KwMacroRules => Some (kw :: MacroRules) , TokenType :: KwMacro => Some (kw :: Macro) , TokenType :: KwMatch => Some (kw :: Match) , TokenType :: KwMod => Some (kw :: Mod) , TokenType :: KwMove => Some (kw :: Move) , TokenType :: KwMut => Some (kw :: Mut) , TokenType :: KwPub => Some (kw :: Pub) , TokenType :: KwRaw => Some (kw :: Raw) , TokenType :: KwRef => Some (kw :: Ref) , TokenType :: KwReturn => Some (kw :: Return) , TokenType :: KwReuse => Some (kw :: Reuse) , TokenType :: KwSafe => Some (kw :: Safe) , TokenType :: KwSelfUpper => Some (kw :: SelfUpper) , TokenType :: KwStatic => Some (kw :: Static) , TokenType :: KwStruct => Some (kw :: Struct) , TokenType :: KwSuper => Some (kw :: Super) , TokenType :: KwTrait => Some (kw :: Trait) , TokenType :: KwTry => Some (kw :: Try) , TokenType :: KwType => Some (kw :: Type) , TokenType :: KwUnderscore => Some (kw :: Underscore) , TokenType :: KwUnsafe => Some (kw :: Unsafe) , TokenType :: KwUse => Some (kw :: Use) , TokenType :: KwWhere => Some (kw :: Where) , TokenType :: KwWhile => Some (kw :: While) , TokenType :: KwYield => Some (kw :: Yield) , TokenType :: SymAttSyntax => Some (sym :: att_syntax) , TokenType :: SymClobberAbi => Some (sym :: clobber_abi) , TokenType :: SymInlateout => Some (sym :: inlateout) , TokenType :: SymInout => Some (sym :: inout) , TokenType :: SymIs => Some (sym :: is) , TokenType :: SymLabel => Some (sym :: label) , TokenType :: SymLateout => Some (sym :: lateout) , TokenType :: SymMayUnwind => Some (sym :: may_unwind) , TokenType :: SymNomem => Some (sym :: nomem) , TokenType :: SymNoreturn => Some (sym :: noreturn) , TokenType :: SymNostack => Some (sym :: nostack) , TokenType :: SymOptions => Some (sym :: options) , TokenType :: SymOut => Some (sym :: out) , TokenType :: SymPreservesFlags => Some (sym :: preserves_flags) , TokenType :: SymPure => Some (sym :: pure) , TokenType :: SymReadonly => Some (sym :: readonly) , TokenType :: SymSym => Some (sym :: sym) , _ => None , } } pub (super) fn to_string (& self) -> String { match self { TokenType :: Eq => "`=`" , TokenType :: Lt => "`<`" , TokenType :: Le => "`<=`" , TokenType :: EqEq => "`==`" , TokenType :: Gt => "`>`" , TokenType :: AndAnd => "`&&`" , TokenType :: OrOr => "`||`" , TokenType :: Bang => "`!`" , TokenType :: Tilde => "`~`" , TokenType :: Plus => "`+`" , TokenType :: Minus => "`-`" , TokenType :: Star => "`*`" , TokenType :: And => "`&`" , TokenType :: Or => "`|`" , TokenType :: At => "`@`" , TokenType :: Dot => "`.`" , TokenType :: DotDot => "`..`" , TokenType :: DotDotDot => "`...`" , TokenType :: DotDotEq => "`..=`" , TokenType :: Comma => "`,`" , TokenType :: Semi => "`;`" , TokenType :: Colon => "`:`" , TokenType :: PathSep => "`::`" , TokenType :: RArrow => "`->`" , TokenType :: FatArrow => "`=>`" , TokenType :: Pound => "`#`" , TokenType :: Question => "`?`" , TokenType :: OpenParen => "`(`" , TokenType :: CloseParen => "`)`" , TokenType :: OpenBrace => "`{`" , TokenType :: CloseBrace => "`}`" , TokenType :: OpenBracket => "`[`" , TokenType :: CloseBracket => "`]`" , TokenType :: Eof => "<eof>" , TokenType :: Operator => "an operator" , TokenType :: Ident => "identifier" , TokenType :: Lifetime => "lifetime" , TokenType :: Path => "path" , TokenType :: Type => "type" , TokenType :: Const => "a const expression" , _ => return format ! ("`{}`" , self . is_keyword () . unwrap ()) , } . to_string () } }}}
mkitem!{mkstruct!{# [doc = " Used by various `Parser` methods such as `check` and `eat`. The first field"] # [doc = " is always by used those methods. The second field is only used when the"] # [doc = " first field doesn't match."] # [derive (Clone , Copy , Debug)] pub struct ExpTokenPair { pub tok : TokenKind , pub token_type : TokenType , }}}
mkitem!{mkstruct!{# [doc = " Used by various `Parser` methods such as `check_keyword` and `eat_keyword`."] # [doc = " The first field is always used by those methods. The second field is only"] # [doc = " used when the first field doesn't match."] # [derive (Clone , Copy)] pub struct ExpKeywordPair { pub kw : Symbol , pub token_type : TokenType , }}}
mkitem!{# [macro_export] # [cfg_attr (rustfmt , rustfmt :: skip)] macro_rules ! exp { (@ tok , $ tok : ident) => { $ crate :: parser :: token_type :: ExpTokenPair { tok : rustc_ast :: token ::$ tok , token_type : $ crate :: parser :: token_type :: TokenType ::$ tok } } ; (@ kw , $ kw : ident , $ token_type : ident) => { $ crate :: parser :: token_type :: ExpKeywordPair { kw : rustc_span :: symbol :: kw ::$ kw , token_type : $ crate :: parser :: token_type :: TokenType ::$ token_type , } } ; (@ sym , $ kw : ident , $ token_type : ident) => { $ crate :: parser :: token_type :: ExpKeywordPair { kw : rustc_span :: symbol :: sym ::$ kw , token_type : $ crate :: parser :: token_type :: TokenType ::$ token_type , } } ; (Eq) => { exp ! (@ tok , Eq) } ; (Lt) => { exp ! (@ tok , Lt) } ; (Le) => { exp ! (@ tok , Le) } ; (EqEq) => { exp ! (@ tok , EqEq) } ; (Gt) => { exp ! (@ tok , Gt) } ; (AndAnd) => { exp ! (@ tok , AndAnd) } ; (OrOr) => { exp ! (@ tok , OrOr) } ; (Bang) => { exp ! (@ tok , Bang) } ; (Tilde) => { exp ! (@ tok , Tilde) } ; (Plus) => { exp ! (@ tok , Plus) } ; (Minus) => { exp ! (@ tok , Minus) } ; (Star) => { exp ! (@ tok , Star) } ; (And) => { exp ! (@ tok , And) } ; (Or) => { exp ! (@ tok , Or) } ; (At) => { exp ! (@ tok , At) } ; (Dot) => { exp ! (@ tok , Dot) } ; (DotDot) => { exp ! (@ tok , DotDot) } ; (DotDotDot) => { exp ! (@ tok , DotDotDot) } ; (DotDotEq) => { exp ! (@ tok , DotDotEq) } ; (Comma) => { exp ! (@ tok , Comma) } ; (Semi) => { exp ! (@ tok , Semi) } ; (Colon) => { exp ! (@ tok , Colon) } ; (PathSep) => { exp ! (@ tok , PathSep) } ; (RArrow) => { exp ! (@ tok , RArrow) } ; (FatArrow) => { exp ! (@ tok , FatArrow) } ; (Pound) => { exp ! (@ tok , Pound) } ; (Question) => { exp ! (@ tok , Question) } ; (Eof) => { exp ! (@ tok , Eof) } ; (OpenParen) => { exp ! (@ tok , OpenParen) } ; (OpenBrace) => { exp ! (@ tok , OpenBrace) } ; (OpenBracket) => { exp ! (@ tok , OpenBracket) } ; (CloseParen) => { exp ! (@ tok , CloseParen) } ; (CloseBrace) => { exp ! (@ tok , CloseBrace) } ; (CloseBracket) => { exp ! (@ tok , CloseBracket) } ; (As) => { exp ! (@ kw , As , KwAs) } ; (Async) => { exp ! (@ kw , Async , KwAsync) } ; (Auto) => { exp ! (@ kw , Auto , KwAuto) } ; (Await) => { exp ! (@ kw , Await , KwAwait) } ; (Become) => { exp ! (@ kw , Become , KwBecome) } ; (Box) => { exp ! (@ kw , Box , KwBox) } ; (Break) => { exp ! (@ kw , Break , KwBreak) } ; (Catch) => { exp ! (@ kw , Catch , KwCatch) } ; (Const) => { exp ! (@ kw , Const , KwConst) } ; (Continue) => { exp ! (@ kw , Continue , KwContinue) } ; (ContractEnsures) => { exp ! (@ kw , ContractEnsures , KwContractEnsures) } ; (ContractRequires) => { exp ! (@ kw , ContractRequires , KwContractRequires) } ; (Crate) => { exp ! (@ kw , Crate , KwCrate) } ; (Default) => { exp ! (@ kw , Default , KwDefault) } ; (Dyn) => { exp ! (@ kw , Dyn , KwDyn) } ; (Else) => { exp ! (@ kw , Else , KwElse) } ; (Enum) => { exp ! (@ kw , Enum , KwEnum) } ; (Extern) => { exp ! (@ kw , Extern , KwExtern) } ; (Fn) => { exp ! (@ kw , Fn , KwFn) } ; (For) => { exp ! (@ kw , For , KwFor) } ; (Gen) => { exp ! (@ kw , Gen , KwGen) } ; (If) => { exp ! (@ kw , If , KwIf) } ; (Impl) => { exp ! (@ kw , Impl , KwImpl) } ; (In) => { exp ! (@ kw , In , KwIn) } ; (Let) => { exp ! (@ kw , Let , KwLet) } ; (Loop) => { exp ! (@ kw , Loop , KwLoop) } ; (Macro) => { exp ! (@ kw , Macro , KwMacro) } ; (MacroRules) => { exp ! (@ kw , MacroRules , KwMacroRules) } ; (Match) => { exp ! (@ kw , Match , KwMatch) } ; (Mod) => { exp ! (@ kw , Mod , KwMod) } ; (Move) => { exp ! (@ kw , Move , KwMove) } ; (Mut) => { exp ! (@ kw , Mut , KwMut) } ; (Pub) => { exp ! (@ kw , Pub , KwPub) } ; (Raw) => { exp ! (@ kw , Raw , KwRaw) } ; (Ref) => { exp ! (@ kw , Ref , KwRef) } ; (Return) => { exp ! (@ kw , Return , KwReturn) } ; (Reuse) => { exp ! (@ kw , Reuse , KwReuse) } ; (Safe) => { exp ! (@ kw , Safe , KwSafe) } ; (SelfUpper) => { exp ! (@ kw , SelfUpper , KwSelfUpper) } ; (Static) => { exp ! (@ kw , Static , KwStatic) } ; (Struct) => { exp ! (@ kw , Struct , KwStruct) } ; (Super) => { exp ! (@ kw , Super , KwSuper) } ; (Trait) => { exp ! (@ kw , Trait , KwTrait) } ; (Try) => { exp ! (@ kw , Try , KwTry) } ; (Type) => { exp ! (@ kw , Type , KwType) } ; (Underscore) => { exp ! (@ kw , Underscore , KwUnderscore) } ; (Unsafe) => { exp ! (@ kw , Unsafe , KwUnsafe) } ; (Use) => { exp ! (@ kw , Use , KwUse) } ; (Where) => { exp ! (@ kw , Where , KwWhere) } ; (While) => { exp ! (@ kw , While , KwWhile) } ; (Yield) => { exp ! (@ kw , Yield , KwYield) } ; (AttSyntax) => { exp ! (@ sym , att_syntax , SymAttSyntax) } ; (ClobberAbi) => { exp ! (@ sym , clobber_abi , SymClobberAbi) } ; (Inlateout) => { exp ! (@ sym , inlateout , SymInlateout) } ; (Inout) => { exp ! (@ sym , inout , SymInout) } ; (Is) => { exp ! (@ sym , is , SymIs) } ; (Label) => { exp ! (@ sym , label , SymLabel) } ; (Lateout) => { exp ! (@ sym , lateout , SymLateout) } ; (MayUnwind) => { exp ! (@ sym , may_unwind , SymMayUnwind) } ; (Nomem) => { exp ! (@ sym , nomem , SymNomem) } ; (Noreturn) => { exp ! (@ sym , noreturn , SymNoreturn) } ; (Nostack) => { exp ! (@ sym , nostack , SymNostack) } ; (Options) => { exp ! (@ sym , options , SymOptions) } ; (Out) => { exp ! (@ sym , out , SymOut) } ; (PreservesFlags) => { exp ! (@ sym , preserves_flags , SymPreservesFlags) } ; (Pure) => { exp ! (@ sym , pure , SymPure) } ; (Readonly) => { exp ! (@ sym , readonly , SymReadonly) } ; (Sym) => { exp ! (@ sym , sym , SymSym) } ; }}
mkitem!{mkstruct!{# [doc = " A bitset type designed specifically for `Parser::expected_token_types`,"] # [doc = " which is very hot. `u128` is the smallest integer that will fit every"] # [doc = " `TokenType` value."] # [derive (Clone , Copy)] pub (super) struct TokenTypeSet (u128) ;}}
mkitem!{mkimpl!{impl TokenTypeSet { pub (super) fn new () -> TokenTypeSet { TokenTypeSet (0) } pub (super) fn is_empty (& self) -> bool { self . 0 == 0 } pub (super) fn insert (& mut self , token_type : TokenType) { self . 0 = self . 0 | (1u128 << token_type as u32) } pub (super) fn clear (& mut self) { self . 0 = 0 } pub (super) fn contains (& self , token_type : TokenType) -> bool { self . 0 & (1u128 << token_type as u32) != 0 } pub (super) fn iter (& self) -> TokenTypeSetIter { TokenTypeSetIter (* self) } }}}
mkitem!{mkstruct!{pub (super) struct TokenTypeSetIter (TokenTypeSet) ;}}
mkitem!{mkimpl!{impl Iterator for TokenTypeSetIter { type Item = TokenType ; fn next (& mut self) -> Option < TokenType > { let num_bits : u32 = (size_of_val (& self . 0 . 0) * 8) as u32 ; assert_eq ! (num_bits , 128) ; let z = self . 0 . 0 . trailing_zeros () ; if z == num_bits { None } else { self . 0 . 0 &= ! (1 << z) ; Some (TokenType :: from_u32 (z)) } } }}}