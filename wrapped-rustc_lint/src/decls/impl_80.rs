macro_rules! deps {
    () => {
        BuiltinEllipsisInclusiveRangePatterns!();
        EarlyContext!();
        EllipsisInclusiveRangePatterns!();
        BuiltinEllipsisInclusiveRangePatternsLint!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl EarlyLintPass for EllipsisInclusiveRangePatterns { fn check_pat (& mut self , cx : & EarlyContext < '_ > , pat : & ast :: Pat) { if self . node_id . is_some () { return ; } use self :: ast :: PatKind ; use self :: ast :: RangeSyntax :: DotDotDot ; # [doc = " If `pat` is a `...` pattern, return the start and end of the range, as well as the span"] # [doc = " corresponding to the ellipsis."] fn matches_ellipsis_pat (pat : & ast :: Pat) -> Option < (Option < & Expr > , & Expr , Span) > { match & pat . kind { PatKind :: Range (a , Some (b) , Spanned { span , node : RangeEnd :: Included (DotDotDot) } ,) => Some ((a . as_deref () , b , * span)) , _ => None , } } let (parentheses , endpoints) = match & pat . kind { PatKind :: Ref (subpat , _) => (true , matches_ellipsis_pat (subpat)) , _ => (false , matches_ellipsis_pat (pat)) , } ; if let Some ((start , end , join)) = endpoints { if parentheses { self . node_id = Some (pat . id) ; let end = expr_to_string (end) ; let replace = match start { Some (start) => format ! ("&({}..={})" , expr_to_string (start) , end) , None => format ! ("&(..={end})") , } ; if join . edition () >= Edition :: Edition2021 { cx . sess () . dcx () . emit_err (BuiltinEllipsisInclusiveRangePatterns { span : pat . span , suggestion : pat . span , replace , }) ; } else { cx . emit_span_lint (ELLIPSIS_INCLUSIVE_RANGE_PATTERNS , pat . span , BuiltinEllipsisInclusiveRangePatternsLint :: Parenthesise { suggestion : pat . span , replace , } ,) ; } } else { let replace = "..=" ; if join . edition () >= Edition :: Edition2021 { cx . sess () . dcx () . emit_err (BuiltinEllipsisInclusiveRangePatterns { span : pat . span , suggestion : join , replace : replace . to_string () , }) ; } else { cx . emit_span_lint (ELLIPSIS_INCLUSIVE_RANGE_PATTERNS , join , BuiltinEllipsisInclusiveRangePatternsLint :: NonParenthesise { suggestion : join , } ,) ; } } ; } } fn check_pat_post (& mut self , _cx : & EarlyContext < '_ > , pat : & ast :: Pat) { if let Some (node_id) = self . node_id { if pat . id == node_id { self . node_id = None } } } }
    };
}

impl_80!();