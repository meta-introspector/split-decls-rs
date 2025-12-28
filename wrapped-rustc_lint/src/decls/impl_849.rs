macro_rules! deps {
    () => {
        UnusedDelimLint!();
        UnusedParens!();
        UnusedDelimsCtx!();
        EarlyContext!();
    };
}

macro_rules! impl_849 {
    () => {
        deps!();
        impl UnusedDelimLint for UnusedParens { const DELIM_STR : & 'static str = "parentheses" ; const LINT_EXPR_IN_PATTERN_MATCHING_CTX : bool = true ; fn lint (& self) -> & 'static Lint { UNUSED_PARENS } fn check_unused_delims_expr (& self , cx : & EarlyContext < '_ > , value : & ast :: Expr , ctx : UnusedDelimsCtx , followed_by_block : bool , left_pos : Option < BytePos > , right_pos : Option < BytePos > , is_kw : bool ,) { match value . kind { ast :: ExprKind :: Paren (ref inner) => { if ! Self :: is_expr_delims_necessary (inner , ctx , followed_by_block) && value . attrs . is_empty () && ! value . span . from_expansion () && (ctx != UnusedDelimsCtx :: LetScrutineeExpr || ! matches ! (inner . kind , ast :: ExprKind :: Binary (rustc_span :: source_map :: Spanned { node , .. } , _ , _ ,) if node . is_lazy ())) && ! ((ctx == UnusedDelimsCtx :: ReturnValue || ctx == UnusedDelimsCtx :: BreakValue) && matches ! (inner . kind , ast :: ExprKind :: Assign (_ , _ , _))) { self . emit_unused_delims_expr (cx , value , ctx , left_pos , right_pos , is_kw) } } ast :: ExprKind :: Let (_ , ref expr , _ , _) => { self . check_unused_delims_expr (cx , expr , UnusedDelimsCtx :: LetScrutineeExpr , followed_by_block , None , None , false ,) ; } _ => { } } } }
    };
}

impl_849!();