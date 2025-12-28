macro_rules! deps {
    () => {
        BuiltinWhileTrue!();
        EarlyContext!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl EarlyLintPass for WhileTrue { # [inline] fn check_expr (& mut self , cx : & EarlyContext < '_ > , e : & ast :: Expr) { if let ast :: ExprKind :: While (cond , _ , label) = & e . kind && let ast :: ExprKind :: Lit (token_lit) = cond . peel_parens () . kind && let token :: Lit { kind : token :: Bool , symbol : kw :: True , .. } = token_lit && ! cond . span . from_expansion () { let condition_span = e . span . with_hi (cond . span . hi ()) ; let replace = format ! ("{}loop" , label . map_or_else (String :: new , | label | format ! ("{}: " , label . ident ,))) ; cx . emit_span_lint (WHILE_TRUE , condition_span , BuiltinWhileTrue { suggestion : condition_span , replace } ,) ; } } }
    };
}

impl_19!()