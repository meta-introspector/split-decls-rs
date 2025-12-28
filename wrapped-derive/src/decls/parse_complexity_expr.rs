macro_rules! deps {
    () => {
        GeneratorResult!();
    };
}

macro_rules! parse_complexity_expr {
    () => {
        deps!();
        pub fn parse_complexity_expr (expr : Expr) -> GeneratorResult < (HashSet < String > , Expr) > { # [derive (Default)] struct VisitComplexityExpr { variables : HashSet < String > , } impl < 'a > Visit < 'a > for VisitComplexityExpr { fn visit_expr_path (& mut self , i : & 'a ExprPath) { if let Some (ident) = i . path . get_ident () { if ident != "child_complexity" { self . variables . insert (ident . to_string ()) ; } } } } let mut visit = VisitComplexityExpr :: default () ; visit . visit_expr (& expr) ; Ok ((visit . variables , expr)) }
    };
}

parse_complexity_expr!();