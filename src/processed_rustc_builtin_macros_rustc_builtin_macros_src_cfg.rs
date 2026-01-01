/* FP:cfg.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cfg_USE_0001
/* FP:cfg.rs-0002 */ use crate :: rustc_complete :: token ;
/* FP:cfg.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cfg_USE_0002
/* FP:cfg.rs-0004 */ use crate :: rustc_complete :: tokenstream :: TokenStream ;
/* FP:cfg.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cfg_USE_0003
/* FP:cfg.rs-0006 */ use crate :: rustc_complete :: PResult ;
/* FP:cfg.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cfg_USE_0004
/* FP:cfg.rs-0008 */ use crate :: rustc_expand :: base :: { DummyResult , ExpandResult , ExtCtxt , MacEager , MacroExpanderResult } ;
/* FP:cfg.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cfg_USE_0005
/* FP:cfg.rs-0010 */ use crate :: rustc_parse :: exp ;
/* FP:cfg.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cfg_USE_0006
/* FP:cfg.rs-0012 */ use crate :: rustc_complete :: Span ;
/* FP:cfg.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cfg_USE_0007
/* FP:cfg.rs-0014 */ use { rustc_ast as ast , rustc_attr_parsing as attr } ;
/* FP:cfg.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cfg_USE_0008
/* FP:cfg.rs-0016 */ use crate :: errors ;
/* FP:cfg.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cfg_FN_0009
/* FP:cfg.rs-0018 */ pub (crate) fn expand_cfg (cx : & mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'static > { let sp = cx . with_def_site_ctxt (sp) ; ExpandResult :: Ready (match parse_cfg (cx , sp , tts) { Ok (cfg) => { let matches_cfg = attr :: cfg_matches (& cfg , & cx . sess , cx . current_expansion . lint_node_id , Some (cx . ecfg . features) ,) ; MacEager :: expr (cx . expr_bool (sp , matches_cfg)) } Err (err) => { let guar = err . emit () ; DummyResult :: any (sp , guar) } }) }
/* FP:cfg.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_cfg_FN_0010
/* FP:cfg.rs-0020 */ fn parse_cfg < 'a > (cx : & ExtCtxt < 'a > , span : Span , tts : TokenStream ,) -> PResult < 'a , ast :: MetaItemInner > { let mut p = cx . new_parser_from_tts (tts) ; if p . token == token :: Eof { return Err (cx . dcx () . create_err (errors :: RequiresCfgPattern { span })) ; } let cfg = p . parse_meta_item_inner () ? ; let _ = p . eat (exp ! (Comma)) ; if ! p . eat (exp ! (Eof)) { return Err (cx . dcx () . create_err (errors :: OneCfgPattern { span })) ; } Ok (cfg) }