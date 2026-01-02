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
mkuse!{use rustc_ast :: token :: { self , Delimiter , IdentIsRaw , Lit , Token , TokenKind } ;}
mkuse!{use rustc_ast :: tokenstream :: { TokenStream , TokenStreamIter , TokenTree } ;}
mkuse!{use rustc_ast :: { LitIntType , LitKind } ;}
mkuse!{use rustc_ast_pretty :: pprust ;}
mkuse!{use rustc_errors :: { Applicability , PResult } ;}
mkuse!{use rustc_macros :: { Decodable , Encodable } ;}
mkuse!{use rustc_session :: parse :: ParseSess ;}
mkuse!{use rustc_span :: { Ident , Span , Symbol } ;}
mkuse!{use crate :: errors ;}
mkitem!{pub (crate) const RAW_IDENT_ERR : & str = "`${concat(..)}` currently does not support raw identifiers" ;}
mkitem!{pub (crate) const UNSUPPORTED_CONCAT_ELEM_ERR : & str = "expected identifier or string literal" ;}
mkitem!{mkenum!{# [doc = " A meta-variable expression, for expansions based on properties of meta-variables."] # [derive (Debug , PartialEq , Encodable , Decodable)] pub (crate) enum MetaVarExpr { # [doc = " Unification of two or more identifiers."] Concat (Box < [MetaVarExprConcatElem] >) , # [doc = " The number of repetitions of an identifier."] Count (Ident , usize) , # [doc = " Ignore a meta-variable for repetition without expansion."] Ignore (Ident) , # [doc = " The index of the repetition at a particular depth, where 0 is the innermost"] # [doc = " repetition. The `usize` is the depth."] Index (usize) , # [doc = " The length of the repetition at a particular depth, where 0 is the innermost"] # [doc = " repetition. The `usize` is the depth."] Len (usize) , }}}
mkitem!{mkimpl!{impl MetaVarExpr { # [doc = " Attempt to parse a meta-variable expression from a token stream."] pub (crate) fn parse < 'psess > (input : & TokenStream , outer_span : Span , psess : & 'psess ParseSess ,) -> PResult < 'psess , MetaVarExpr > { let mut iter = input . iter () ; let ident = parse_ident (& mut iter , psess , outer_span) ? ; let next = iter . next () ; let Some (TokenTree :: Delimited (.. , Delimiter :: Parenthesis , args)) = next else { let (unexpected_span , insert_span) = match next { Some (TokenTree :: Delimited (..)) => (None , None) , Some (tt) => (Some (tt . span ()) , None) , None => (None , Some (ident . span . shrink_to_hi ())) , } ; let err = errors :: MveMissingParen { ident_span : ident . span , unexpected_span , insert_span } ; return Err (psess . dcx () . create_err (err)) ; } ; if iter . peek () . is_some () { let span = iter_span (& iter) . expect ("checked is_some above") ; let err = errors :: MveExtraTokens { span , ident_span : ident . span , extra_count : iter . count () , .. Default :: default () } ; return Err (psess . dcx () . create_err (err)) ; } let mut iter = args . iter () ; let rslt = match ident . as_str () { "concat" => parse_concat (& mut iter , psess , outer_span , ident . span) ? , "count" => parse_count (& mut iter , psess , ident . span) ? , "ignore" => { eat_dollar (& mut iter , psess , ident . span) ? ; MetaVarExpr :: Ignore (parse_ident (& mut iter , psess , ident . span) ?) } "index" => MetaVarExpr :: Index (parse_depth (& mut iter , psess , ident . span) ?) , "len" => MetaVarExpr :: Len (parse_depth (& mut iter , psess , ident . span) ?) , _ => { let err = errors :: MveUnrecognizedExpr { span : ident . span , valid_expr_list : "`count`, `ignore`, `index`, `len`, and `concat`" , } ; return Err (psess . dcx () . create_err (err)) ; } } ; check_trailing_tokens (& mut iter , psess , ident) ? ; Ok (rslt) } pub (crate) fn for_each_metavar < A > (& self , mut aux : A , mut cb : impl FnMut (A , & Ident) -> A) -> A { match self { MetaVarExpr :: Concat (elems) => { for elem in elems { if let MetaVarExprConcatElem :: Var (ident) = elem { aux = cb (aux , ident) } } aux } MetaVarExpr :: Count (ident , _) | MetaVarExpr :: Ignore (ident) => cb (aux , ident) , MetaVarExpr :: Index (..) | MetaVarExpr :: Len (..) => aux , } } }}}

macro_rules! check_trailing_tokens_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_trailing_tokens in module {}", module_path!());
    };
}

mkfn!{
    check_trailing_tokens_introspect!();
    # [doc = " Checks if there are any remaining tokens (for example, `${ignore($valid, extra)}`) and create"] # [doc = " a diag with the correct arg count if so."] fn check_trailing_tokens < 'psess > (iter : & mut TokenStreamIter < '_ > , psess : & 'psess ParseSess , ident : Ident ,) -> PResult < 'psess , () > { if iter . peek () . is_none () { return Ok (()) ; } let (min_or_exact_args , max_args) = match ident . as_str () { "concat" => panic ! ("concat takes unlimited tokens but didn't eat them all") , "ignore" => (1 , None) , "count" => (1 , Some (2)) , "index" => (0 , Some (1)) , "len" => (0 , Some (1)) , other => unreachable ! ("unknown MVEs should be rejected earlier (got `{other}`)") , } ; let err = errors :: MveExtraTokens { span : iter_span (iter) . expect ("checked is_none above") , ident_span : ident . span , extra_count : iter . count () , exact_args_note : if max_args . is_some () { None } else { Some (()) } , range_args_note : if max_args . is_some () { Some (()) } else { None } , min_or_exact_args , max_args : max_args . unwrap_or_default () , name : ident . to_string () , } ; Err (psess . dcx () . create_err (err)) }
}

macro_rules! iter_span_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function iter_span in module {}", module_path!());
    };
}

mkfn!{
    iter_span_introspect!();
    # [doc = " Returns a span encompassing all tokens in the iterator if there is at least one item."] fn iter_span (iter : & TokenStreamIter < '_ >) -> Option < Span > { let mut iter = iter . clone () ; let first_sp = iter . next () ? . span () ; let last_sp = iter . last () . map (TokenTree :: span) . unwrap_or (first_sp) ; let span = first_sp . with_hi (last_sp . hi ()) ; Some (span) }
}
mkitem!{mkenum!{# [doc = " Indicates what is placed in a `concat` parameter. For example, literals"] # [doc = " (`${concat(\"foo\", \"bar\")}`) or adhoc identifiers (`${concat(foo, bar)}`)."] # [derive (Debug , Decodable , Encodable , PartialEq)] pub (crate) enum MetaVarExprConcatElem { # [doc = " Identifier WITHOUT a preceding dollar sign, which means that this identifier should be"] # [doc = " interpreted as a literal."] Ident (Ident) , # [doc = " For example, a number or a string."] Literal (Symbol) , # [doc = " Identifier WITH a preceding dollar sign, which means that this identifier should be"] # [doc = " expanded and interpreted as a variable."] Var (Ident) , }}}

macro_rules! parse_concat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_concat in module {}", module_path!());
    };
}

mkfn!{
    parse_concat_introspect!();
    # [doc = " Parse a meta-variable `concat` expression: `concat($metavar, ident, ...)`."] fn parse_concat < 'psess > (iter : & mut TokenStreamIter < '_ > , psess : & 'psess ParseSess , outer_span : Span , expr_ident_span : Span ,) -> PResult < 'psess , MetaVarExpr > { let mut result = Vec :: new () ; loop { let is_var = try_eat_dollar (iter) ; let token = parse_token (iter , psess , outer_span) ? ; let element = if is_var { MetaVarExprConcatElem :: Var (parse_ident_from_token (psess , token) ?) } else if let TokenKind :: Literal (Lit { kind : token :: LitKind :: Str , symbol , suffix : None }) = token . kind { MetaVarExprConcatElem :: Literal (symbol) } else { match parse_ident_from_token (psess , token) { Err (err) => { err . cancel () ; return Err (psess . dcx () . struct_span_err (token . span , UNSUPPORTED_CONCAT_ELEM_ERR)) ; } Ok (elem) => MetaVarExprConcatElem :: Ident (elem) , } } ; result . push (element) ; if iter . peek () . is_none () { break ; } if ! try_eat_comma (iter) { return Err (psess . dcx () . struct_span_err (outer_span , "expected comma")) ; } } if result . len () < 2 { return Err (psess . dcx () . struct_span_err (expr_ident_span , "`concat` must have at least two elements")) ; } Ok (MetaVarExpr :: Concat (result . into ())) }
}

macro_rules! parse_count_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_count in module {}", module_path!());
    };
}

mkfn!{
    parse_count_introspect!();
    # [doc = " Parse a meta-variable `count` expression: `count(ident[, depth])`"] fn parse_count < 'psess > (iter : & mut TokenStreamIter < '_ > , psess : & 'psess ParseSess , span : Span ,) -> PResult < 'psess , MetaVarExpr > { eat_dollar (iter , psess , span) ? ; let ident = parse_ident (iter , psess , span) ? ; let depth = if try_eat_comma (iter) { if iter . peek () . is_none () { return Err (psess . dcx () . struct_span_err (span , "`count` followed by a comma must have an associated index indicating its depth" ,)) ; } parse_depth (iter , psess , span) ? } else { 0 } ; Ok (MetaVarExpr :: Count (ident , depth)) }
}

macro_rules! parse_depth_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_depth in module {}", module_path!());
    };
}

mkfn!{
    parse_depth_introspect!();
    # [doc = " Parses the depth used by index(depth) and len(depth)."] fn parse_depth < 'psess > (iter : & mut TokenStreamIter < '_ > , psess : & 'psess ParseSess , span : Span ,) -> PResult < 'psess , usize > { let Some (tt) = iter . next () else { return Ok (0) } ; let TokenTree :: Token (Token { kind : TokenKind :: Literal (lit) , .. } , _) = tt else { return Err (psess . dcx () . struct_span_err (span , "meta-variable expression depth must be a literal")) ; } ; if let Ok (lit_kind) = LitKind :: from_token_lit (* lit) && let LitKind :: Int (n_u128 , LitIntType :: Unsuffixed) = lit_kind && let Ok (n_usize) = usize :: try_from (n_u128 . get ()) { Ok (n_usize) } else { let msg = "only unsuffixes integer literals are supported in meta-variable expressions" ; Err (psess . dcx () . struct_span_err (span , msg)) } }
}

macro_rules! parse_ident_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_ident in module {}", module_path!());
    };
}

mkfn!{
    parse_ident_introspect!();
    # [doc = " Parses an generic ident"] fn parse_ident < 'psess > (iter : & mut TokenStreamIter < '_ > , psess : & 'psess ParseSess , fallback_span : Span ,) -> PResult < 'psess , Ident > { let token = parse_token (iter , psess , fallback_span) ? ; parse_ident_from_token (psess , token) }
}

macro_rules! parse_ident_from_token_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_ident_from_token in module {}", module_path!());
    };
}

mkfn!{
    parse_ident_from_token_introspect!();
    fn parse_ident_from_token < 'psess > (psess : & 'psess ParseSess , token : & Token ,) -> PResult < 'psess , Ident > { if let Some ((elem , is_raw)) = token . ident () { if let IdentIsRaw :: Yes = is_raw { return Err (psess . dcx () . struct_span_err (elem . span , RAW_IDENT_ERR)) ; } return Ok (elem) ; } let token_str = pprust :: token_to_string (token) ; let mut err = psess . dcx () . struct_span_err (token . span , format ! ("expected identifier, found `{token_str}`")) ; err . span_suggestion (token . span , format ! ("try removing `{token_str}`") , "" , Applicability :: MaybeIncorrect ,) ; Err (err) }
}

macro_rules! parse_token_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_token in module {}", module_path!());
    };
}

mkfn!{
    parse_token_introspect!();
    fn parse_token < 'psess , 't > (iter : & mut TokenStreamIter < 't > , psess : & 'psess ParseSess , fallback_span : Span ,) -> PResult < 'psess , & 't Token > { let Some (tt) = iter . next () else { return Err (psess . dcx () . struct_span_err (fallback_span , UNSUPPORTED_CONCAT_ELEM_ERR)) ; } ; let TokenTree :: Token (token , _) = tt else { return Err (psess . dcx () . struct_span_err (tt . span () , UNSUPPORTED_CONCAT_ELEM_ERR)) ; } ; Ok (token) }
}

macro_rules! try_eat_comma_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_eat_comma in module {}", module_path!());
    };
}

mkfn!{
    try_eat_comma_introspect!();
    # [doc = " Tries to move the iterator forward returning `true` if there is a comma. If not, then the"] # [doc = " iterator is not modified and the result is `false`."] fn try_eat_comma (iter : & mut TokenStreamIter < '_ >) -> bool { if let Some (TokenTree :: Token (Token { kind : token :: Comma , .. } , _)) = iter . peek () { let _ = iter . next () ; return true ; } false }
}

macro_rules! try_eat_dollar_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_eat_dollar in module {}", module_path!());
    };
}

mkfn!{
    try_eat_dollar_introspect!();
    # [doc = " Tries to move the iterator forward returning `true` if there is a dollar sign. If not, then the"] # [doc = " iterator is not modified and the result is `false`."] fn try_eat_dollar (iter : & mut TokenStreamIter < '_ >) -> bool { if let Some (TokenTree :: Token (Token { kind : token :: Dollar , .. } , _)) = iter . peek () { let _ = iter . next () ; return true ; } false }
}

macro_rules! eat_dollar_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function eat_dollar in module {}", module_path!());
    };
}

mkfn!{
    eat_dollar_introspect!();
    # [doc = " Expects that the next item is a dollar sign."] fn eat_dollar < 'psess > (iter : & mut TokenStreamIter < '_ > , psess : & 'psess ParseSess , span : Span ,) -> PResult < 'psess , () > { if try_eat_dollar (iter) { return Ok (()) ; } Err (psess . dcx () . struct_span_err (span , "meta-variables within meta-variable expressions must be referenced using a dollar sign" ,)) }
}