/* FP:pattern_type.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_pattern_type_USE_0001
/* FP:pattern_type.rs-0002 */ use crate :: rustc_complete :: tokenstream :: TokenStream ;
/* FP:pattern_type.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_pattern_type_USE_0002
/* FP:pattern_type.rs-0004 */ use crate :: rustc_complete :: { AnonConst , DUMMY_NODE_ID , Ty , TyPat , TyPatKind , ast , token } ;
/* FP:pattern_type.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_pattern_type_USE_0003
/* FP:pattern_type.rs-0006 */ use crate :: rustc_complete :: PResult ;
/* FP:pattern_type.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_pattern_type_USE_0004
/* FP:pattern_type.rs-0008 */ use crate :: rustc_expand :: base :: { self , DummyResult , ExpandResult , ExtCtxt , MacroExpanderResult } ;
/* FP:pattern_type.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_pattern_type_USE_0005
/* FP:pattern_type.rs-0010 */ use crate :: rustc_parse :: exp ;
/* FP:pattern_type.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_pattern_type_USE_0006
/* FP:pattern_type.rs-0012 */ use crate :: rustc_parse :: parser :: { CommaRecoveryMode , RecoverColon , RecoverComma } ;
/* FP:pattern_type.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_pattern_type_USE_0007
/* FP:pattern_type.rs-0014 */ use crate :: rustc_complete :: Span ;
/* FP:pattern_type.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_pattern_type_FN_0008
/* FP:pattern_type.rs-0016 */ pub (crate) fn expand < 'cx > (cx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { let (ty , pat) = match parse_pat_ty (cx , tts) { Ok (parsed) => parsed , Err (err) => { return ExpandResult :: Ready (DummyResult :: any (sp , err . emit ())) ; } } ; ExpandResult :: Ready (base :: MacEager :: ty (cx . ty (sp , ast :: TyKind :: Pat (ty , pat)))) }
/* FP:pattern_type.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_pattern_type_FN_0009
/* FP:pattern_type.rs-0018 */ fn parse_pat_ty < 'a > (cx : & mut ExtCtxt < 'a > , stream : TokenStream ,) -> PResult < 'a , (Box < Ty > , Box < TyPat >) > { let mut parser = cx . new_parser_from_tts (stream) ; let ty = parser . parse_ty () ? ; parser . expect_keyword (exp ! (Is)) ? ; let pat = pat_to_ty_pat (cx , * parser . parse_pat_no_top_guard (None , RecoverComma :: No , RecoverColon :: No , CommaRecoveryMode :: EitherTupleOrPipe ,) ? ,) ; if parser . token != token :: Eof { parser . unexpected () ? ; } Ok ((ty , pat)) }
/* FP:pattern_type.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_pattern_type_FN_0010
/* FP:pattern_type.rs-0020 */ fn ty_pat (kind : TyPatKind , span : Span) -> Box < TyPat > { Box :: new (TyPat { id : DUMMY_NODE_ID , kind , span , tokens : None }) }
/* FP:pattern_type.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_builtin_macros_src_pattern_type_FN_0011
/* FP:pattern_type.rs-0022 */ fn pat_to_ty_pat (cx : & mut ExtCtxt < '_ > , pat : ast :: Pat) -> Box < TyPat > { let kind = match pat . kind { ast :: PatKind :: Range (start , end , include_end) => TyPatKind :: Range (start . map (| value | Box :: new (AnonConst { id : DUMMY_NODE_ID , value })) , end . map (| value | Box :: new (AnonConst { id : DUMMY_NODE_ID , value })) , include_end ,) , ast :: PatKind :: Or (variants) => { TyPatKind :: Or (variants . into_iter () . map (| pat | pat_to_ty_pat (cx , * pat)) . collect ()) } ast :: PatKind :: Err (guar) => TyPatKind :: Err (guar) , _ => TyPatKind :: Err (cx . dcx () . span_err (pat . span , "pattern not supported in pattern types")) , } ; ty_pat (kind , pat . span) }