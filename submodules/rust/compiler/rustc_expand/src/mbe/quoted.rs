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
mkuse!{use rustc_ast :: token :: { self , Delimiter , IdentIsRaw , NonterminalKind , Token } ;}
mkuse!{use rustc_ast :: tokenstream :: TokenStreamIter ;}
mkuse!{use rustc_ast :: { NodeId , tokenstream } ;}
mkuse!{use rustc_ast_pretty :: pprust ;}
mkuse!{use rustc_feature :: Features ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: parse :: feature_err ;}
mkuse!{use rustc_span :: edition :: Edition ;}
mkuse!{use rustc_span :: { Ident , Span , kw , sym } ;}
mkuse!{use crate :: errors ;}
mkuse!{use crate :: mbe :: macro_parser :: count_metavar_decls ;}
mkuse!{use crate :: mbe :: { Delimited , KleeneOp , KleeneToken , MetaVarExpr , SequenceRepetition , TokenTree } ;}
mkitem!{pub (crate) const VALID_FRAGMENT_NAMES_MSG : & str = "valid fragment specifiers are \
    `ident`, `block`, `stmt`, `expr`, `pat`, `ty`, `lifetime`, `literal`, `path`, \
    `meta`, `tt`, `item` and `vis`, along with `expr_2021` and `pat_param` for edition compatibility" ;}
mkitem!{mkenum!{# [doc = " Which part of a macro rule we're parsing"] # [derive (Copy , Clone)] pub (crate) enum RulePart { # [doc = " The left-hand side, with patterns and metavar definitions with types"] Pattern , # [doc = " The right-hand side body, with metavar references and metavar expressions"] Body , }}}
mkitem!{mkimpl!{impl RulePart { # [inline (always)] fn is_pattern (& self) -> bool { matches ! (self , Self :: Pattern) } # [inline (always)] fn is_body (& self) -> bool { matches ! (self , Self :: Body) } }}}

macro_rules! parse_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse in module {}", module_path!());
    };
}

mkfn!{
    parse_introspect!();
    # [doc = " Takes a `tokenstream::TokenStream` and returns a `Vec<self::TokenTree>`. Specifically, this"] # [doc = " takes a generic `TokenStream`, such as is used in the rest of the compiler, and returns a"] # [doc = " collection of `TokenTree` for use in parsing a macro."] # [doc = ""] # [doc = " # Parameters"] # [doc = ""] # [doc = " - `input`: a token stream to read from, the contents of which we are parsing."] # [doc = " - `part`: whether we're parsing the patterns or the body of a macro. Both take roughly the same"] # [doc = "   form _except_ that:"] # [doc = "   - In a pattern, metavars are declared with their \"matcher\" type. For example `$var:expr` or"] # [doc = "     `$id:ident`. In this example, `expr` and `ident` are \"matchers\". They are not present in the"] # [doc = "     body of a macro rule -- just in the pattern."] # [doc = "   - Metavariable expressions are only valid in the \"body\", not the \"pattern\"."] # [doc = " - `sess`: the parsing session. Any errors will be emitted to this session."] # [doc = " - `node_id`: the NodeId of the macro we are parsing."] # [doc = " - `features`: language features so we can do feature gating."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " A collection of `self::TokenTree`. There may also be some errors emitted to `sess`."] fn parse (input : & tokenstream :: TokenStream , part : RulePart , sess : & Session , node_id : NodeId , features : & Features , edition : Edition ,) -> Vec < TokenTree > { let mut result = Vec :: new () ; let mut iter = input . iter () ; while let Some (tree) = iter . next () { let tree = parse_tree (tree , & mut iter , part , sess , node_id , features , edition) ; if part . is_body () { result . push (tree) ; continue ; } let TokenTree :: MetaVar (start_sp , ident) = tree else { result . push (tree) ; continue ; } ; let mut missing_fragment_specifier = | span | { sess . dcx () . emit_err (errors :: MissingFragmentSpecifier { span , add_span : span . shrink_to_hi () , valid : VALID_FRAGMENT_NAMES_MSG , }) ; result . push (TokenTree :: MetaVarDecl { span , name : ident , kind : NonterminalKind :: TT }) ; } ; if let Some (peek) = iter . peek () && let tokenstream :: TokenTree :: Token (token , _spacing) = peek && let Token { kind : token :: Colon , span : colon_span } = token { iter . next () ; let Some (tokenstream :: TokenTree :: Token (token , _)) = iter . next () else { missing_fragment_specifier (colon_span . with_lo (start_sp . lo ())) ; continue ; } ; let Some ((fragment , _)) = token . ident () else { missing_fragment_specifier (token . span) ; continue ; } ; let span = token . span . with_lo (start_sp . lo ()) ; let edition = | | { if ! span . from_expansion () { edition } else { span . edition () } } ; let kind = NonterminalKind :: from_symbol (fragment . name , edition) . unwrap_or_else (| | { sess . dcx () . emit_err (errors :: InvalidFragmentSpecifier { span , fragment , help : VALID_FRAGMENT_NAMES_MSG , }) ; NonterminalKind :: TT }) ; result . push (TokenTree :: MetaVarDecl { span , name : ident , kind }) ; } else { missing_fragment_specifier (start_sp) ; } } result }
}

macro_rules! parse_one_tt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_one_tt in module {}", module_path!());
    };
}

mkfn!{
    parse_one_tt_introspect!();
    # [doc = " Takes a `tokenstream::TokenTree` and returns a `self::TokenTree`. Like `parse`, but for a"] # [doc = " single token tree. Emits errors to `sess` if needed."] # [inline] pub (super) fn parse_one_tt (input : tokenstream :: TokenTree , part : RulePart , sess : & Session , node_id : NodeId , features : & Features , edition : Edition ,) -> TokenTree { parse (& tokenstream :: TokenStream :: new (vec ! [input]) , part , sess , node_id , features , edition) . pop () . unwrap () }
}

macro_rules! maybe_emit_macro_metavar_expr_feature_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function maybe_emit_macro_metavar_expr_feature in module {}", module_path!());
    };
}

mkfn!{
    maybe_emit_macro_metavar_expr_feature_introspect!();
    # [doc = " Asks for the `macro_metavar_expr` feature if it is not enabled"] fn maybe_emit_macro_metavar_expr_feature (features : & Features , sess : & Session , span : Span) { if ! features . macro_metavar_expr () { let msg = "meta-variable expressions are unstable" ; feature_err (sess , sym :: macro_metavar_expr , span , msg) . emit () ; } }
}

macro_rules! maybe_emit_macro_metavar_expr_concat_feature_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function maybe_emit_macro_metavar_expr_concat_feature in module {}", module_path!());
    };
}

mkfn!{
    maybe_emit_macro_metavar_expr_concat_feature_introspect!();
    fn maybe_emit_macro_metavar_expr_concat_feature (features : & Features , sess : & Session , span : Span) { if ! features . macro_metavar_expr_concat () { let msg = "the `concat` meta-variable expression is unstable" ; feature_err (sess , sym :: macro_metavar_expr_concat , span , msg) . emit () ; } }
}

macro_rules! parse_tree_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_tree in module {}", module_path!());
    };
}

mkfn!{
    parse_tree_introspect!();
    # [doc = " Takes a `tokenstream::TokenTree` and returns a `self::TokenTree`. Specifically, this takes a"] # [doc = " generic `TokenTree`, such as is used in the rest of the compiler, and returns a `TokenTree`"] # [doc = " for use in parsing a macro."] # [doc = ""] # [doc = " Converting the given tree may involve reading more tokens."] # [doc = ""] # [doc = " # Parameters"] # [doc = ""] # [doc = " - `tree`: the tree we wish to convert."] # [doc = " - `outer_iter`: an iterator over trees. We may need to read more tokens from it in order to finish"] # [doc = "   converting `tree`"] # [doc = " - `part`: same as [parse]."] # [doc = " - `sess`: the parsing session. Any errors will be emitted to this session."] # [doc = " - `features`: language features so we can do feature gating."] fn parse_tree < 'a > (tree : & 'a tokenstream :: TokenTree , outer_iter : & mut TokenStreamIter < 'a > , part : RulePart , sess : & Session , node_id : NodeId , features : & Features , edition : Edition ,) -> TokenTree { match tree { & tokenstream :: TokenTree :: Token (Token { kind : token :: Dollar , span : dollar_span } , _) => { let mut next = outer_iter . next () ; let mut iter_storage ; let mut iter : & mut TokenStreamIter < '_ > = match next { Some (tokenstream :: TokenTree :: Delimited (.. , delim , tts)) if delim . skip () => { iter_storage = tts . iter () ; next = iter_storage . next () ; & mut iter_storage } _ => outer_iter , } ; match next { Some (& tokenstream :: TokenTree :: Delimited (delim_span , _ , delim , ref tts)) => { if part . is_pattern () { if delim != Delimiter :: Parenthesis { span_dollar_dollar_or_metavar_in_the_lhs_err (sess , & Token { kind : delim . as_open_token_kind () , span : delim_span . entire () , } ,) ; } } else { match delim { Delimiter :: Brace => { match MetaVarExpr :: parse (tts , delim_span . entire () , & sess . psess) { Err (err) => { err . emit () ; return TokenTree :: token (token :: Dollar , dollar_span) ; } Ok (elem) => { if let MetaVarExpr :: Concat (_) = elem { maybe_emit_macro_metavar_expr_concat_feature (features , sess , delim_span . entire () ,) ; } else { maybe_emit_macro_metavar_expr_feature (features , sess , delim_span . entire () ,) ; } return TokenTree :: MetaVarExpr (delim_span , elem) ; } } } Delimiter :: Parenthesis => { } _ => { let token = pprust :: token_kind_to_string (& delim . as_open_token_kind ()) ; sess . dcx () . emit_err (errors :: ExpectedParenOrBrace { span : delim_span . entire () , token , }) ; } } } let sequence = parse (tts , part , sess , node_id , features , edition) ; let (separator , kleene) = parse_sep_and_kleene_op (& mut iter , delim_span . entire () , sess) ; let num_captures = if part . is_pattern () { count_metavar_decls (& sequence) } else { 0 } ; TokenTree :: Sequence (delim_span , SequenceRepetition { tts : sequence , separator , kleene , num_captures } ,) } Some (tokenstream :: TokenTree :: Token (token , _)) if token . is_ident () => { let (ident , is_raw) = token . ident () . unwrap () ; let span = ident . span . with_lo (dollar_span . lo ()) ; if ident . name == kw :: Crate && matches ! (is_raw , IdentIsRaw :: No) { TokenTree :: token (token :: Ident (kw :: DollarCrate , is_raw) , span) } else { TokenTree :: MetaVar (span , ident) } } Some (& tokenstream :: TokenTree :: Token (Token { kind : token :: Dollar , span : dollar_span2 } , _ ,)) => { if part . is_pattern () { span_dollar_dollar_or_metavar_in_the_lhs_err (sess , & Token { kind : token :: Dollar , span : dollar_span2 } ,) ; } else { maybe_emit_macro_metavar_expr_feature (features , sess , dollar_span2) ; } TokenTree :: token (token :: Dollar , dollar_span2) } Some (tokenstream :: TokenTree :: Token (token , _)) => { let msg = format ! ("expected identifier, found `{}`" , pprust :: token_to_string (token) ,) ; sess . dcx () . span_err (token . span , msg) ; TokenTree :: MetaVar (token . span , Ident :: dummy ()) } None => TokenTree :: token (token :: Dollar , dollar_span) , } } tokenstream :: TokenTree :: Token (token , _) => TokenTree :: Token (* token) , & tokenstream :: TokenTree :: Delimited (span , spacing , delim , ref tts) => TokenTree :: Delimited (span , spacing , Delimited { delim , tts : parse (tts , part , sess , node_id , features , edition) } ,) , } }
}

macro_rules! kleene_op_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function kleene_op in module {}", module_path!());
    };
}

mkfn!{
    kleene_op_introspect!();
    # [doc = " Takes a token and returns `Some(KleeneOp)` if the token is `+` `*` or `?`. Otherwise, return"] # [doc = " `None`."] fn kleene_op (token : & Token) -> Option < KleeneOp > { match token . kind { token :: Star => Some (KleeneOp :: ZeroOrMore) , token :: Plus => Some (KleeneOp :: OneOrMore) , token :: Question => Some (KleeneOp :: ZeroOrOne) , _ => None , } }
}

macro_rules! parse_kleene_op_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_kleene_op in module {}", module_path!());
    };
}

mkfn!{
    parse_kleene_op_introspect!();
    # [doc = " Parse the next token tree of the input looking for a KleeneOp. Returns"] # [doc = ""] # [doc = " - Ok(Ok((op, span))) if the next token tree is a KleeneOp"] # [doc = " - Ok(Err(tok, span)) if the next token tree is a token but not a KleeneOp"] # [doc = " - Err(span) if the next token tree is not a token"] fn parse_kleene_op (iter : & mut TokenStreamIter < '_ > , span : Span ,) -> Result < Result < (KleeneOp , Span) , Token > , Span > { match iter . next () { Some (tokenstream :: TokenTree :: Token (token , _)) => match kleene_op (token) { Some (op) => Ok (Ok ((op , token . span))) , None => Ok (Err (* token)) , } , tree => Err (tree . map_or (span , tokenstream :: TokenTree :: span)) , } }
}

macro_rules! parse_sep_and_kleene_op_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_sep_and_kleene_op in module {}", module_path!());
    };
}

mkfn!{
    parse_sep_and_kleene_op_introspect!();
    # [doc = " Attempt to parse a single Kleene star, possibly with a separator."] # [doc = ""] # [doc = " For example, in a pattern such as `$(a),*`, `a` is the pattern to be repeated, `,` is the"] # [doc = " separator, and `*` is the Kleene operator. This function is specifically concerned with parsing"] # [doc = " the last two tokens of such a pattern: namely, the optional separator and the Kleene operator"] # [doc = " itself. Note that here we are parsing the _macro_ itself, rather than trying to match some"] # [doc = " stream of tokens in an invocation of a macro."] # [doc = ""] # [doc = " This function will take some input iterator `iter` corresponding to `span` and a parsing"] # [doc = " session `sess`. If the next one (or possibly two) tokens in `iter` correspond to a Kleene"] # [doc = " operator and separator, then a tuple with `(separator, KleeneOp)` is returned. Otherwise, an"] # [doc = " error with the appropriate span is emitted to `sess` and a dummy value is returned."] fn parse_sep_and_kleene_op (iter : & mut TokenStreamIter < '_ > , span : Span , sess : & Session ,) -> (Option < Token > , KleeneToken) { let span = match parse_kleene_op (iter , span) { Ok (Ok ((op , span))) => return (None , KleeneToken :: new (op , span)) , Ok (Err (token)) => match parse_kleene_op (iter , token . span) { Ok (Ok ((KleeneOp :: ZeroOrOne , span))) => { sess . dcx () . span_err (token . span , "the `?` macro repetition operator does not take a separator" ,) ; return (None , KleeneToken :: new (KleeneOp :: ZeroOrMore , span)) ; } Ok (Ok ((op , span))) => return (Some (token) , KleeneToken :: new (op , span)) , Ok (Err (Token { span , .. })) | Err (span) => span , } , Err (span) => span , } ; sess . dcx () . span_err (span , "expected one of: `*`, `+`, or `?`") ; (None , KleeneToken :: new (KleeneOp :: ZeroOrMore , span)) }
}

macro_rules! span_dollar_dollar_or_metavar_in_the_lhs_err_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function span_dollar_dollar_or_metavar_in_the_lhs_err in module {}", module_path!());
    };
}

mkfn!{
    span_dollar_dollar_or_metavar_in_the_lhs_err_introspect!();
    fn span_dollar_dollar_or_metavar_in_the_lhs_err (sess : & Session , token : & Token) { sess . dcx () . span_err (token . span , format ! ("unexpected token: {}" , pprust :: token_to_string (token))) ; sess . dcx () . span_note (token . span , "`$$` and meta-variable expressions are not allowed inside macro parameter definitions" ,) ; }
}