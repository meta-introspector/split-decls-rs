macro_rules! deps {
    () => {
        Mode!();
        ExprKind!();
        MetaVarKind!();
        ParseError!();
    };
}

macro_rules! eat_fragment_kind {
    () => {
        deps!();
        fn eat_fragment_kind (edition : impl Copy + Fn (SyntaxContext) -> Edition , src : & mut TtIter < '_ , Span > , mode : Mode ,) -> Result < Option < MetaVarKind > , ParseError > { if let Mode :: Pattern = mode { src . expect_char (':') . map_err (| () | ParseError :: unexpected ("missing fragment specifier")) ? ; let ident = src . expect_ident () . map_err (| () | ParseError :: unexpected ("missing fragment specifier")) ? ; let kind = match ident . sym . as_str () { "path" => MetaVarKind :: Path , "ty" => MetaVarKind :: Ty , "pat" => { if edition (ident . span . ctx) . at_least_2021 () { MetaVarKind :: Pat } else { MetaVarKind :: PatParam } } "pat_param" => MetaVarKind :: PatParam , "stmt" => MetaVarKind :: Stmt , "block" => MetaVarKind :: Block , "meta" => MetaVarKind :: Meta , "item" => MetaVarKind :: Item , "vis" => MetaVarKind :: Vis , "expr" => { if edition (ident . span . ctx) . at_least_2024 () { MetaVarKind :: Expr (ExprKind :: Expr) } else { MetaVarKind :: Expr (ExprKind :: Expr2021) } } "expr_2021" => MetaVarKind :: Expr (ExprKind :: Expr2021) , "ident" => MetaVarKind :: Ident , "tt" => MetaVarKind :: Tt , "lifetime" => MetaVarKind :: Lifetime , "literal" => MetaVarKind :: Literal , _ => return Ok (None) , } ; return Ok (Some (kind)) ; } ; Ok (None) }
    };
}

eat_fragment_kind!()