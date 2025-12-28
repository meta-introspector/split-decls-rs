macro_rules! deps {
    () => {
        UnusedDelimLint!();
        EarlyContext!();
        UnusedDelimsCtx!();
    };
}

macro_rules! impl_854 {
    () => {
        deps!();
        impl UnusedDelimLint for UnusedBraces { const DELIM_STR : & 'static str = "braces" ; const LINT_EXPR_IN_PATTERN_MATCHING_CTX : bool = false ; fn lint (& self) -> & 'static Lint { UNUSED_BRACES } fn check_unused_delims_expr (& self , cx : & EarlyContext < '_ > , value : & ast :: Expr , ctx : UnusedDelimsCtx , followed_by_block : bool , left_pos : Option < BytePos > , right_pos : Option < BytePos > , is_kw : bool ,) { match value . kind { ast :: ExprKind :: Block (ref inner , None) if inner . rules == ast :: BlockCheckMode :: Default => { if let [stmt] = inner . stmts . as_slice () && let ast :: StmtKind :: Expr (ref expr) = stmt . kind && ! Self :: is_expr_delims_necessary (expr , ctx , followed_by_block) && (ctx != UnusedDelimsCtx :: AnonConst || (matches ! (expr . kind , ast :: ExprKind :: Lit (_)) && ! expr . span . from_expansion ())) && ctx != UnusedDelimsCtx :: ClosureBody && ! cx . sess () . source_map () . is_multiline (value . span) && value . attrs . is_empty () && ! value . span . from_expansion () && ! inner . span . from_expansion () { self . emit_unused_delims_expr (cx , value , ctx , left_pos , right_pos , is_kw) } } ast :: ExprKind :: Let (_ , ref expr , _ , _) => { self . check_unused_delims_expr (cx , expr , UnusedDelimsCtx :: LetScrutineeExpr , followed_by_block , None , None , false ,) ; } _ => { } } } }
    };
}

impl_854!();