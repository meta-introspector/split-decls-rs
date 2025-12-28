macro_rules! deps {
    () => {
        Pat!();
        Expr!();
    };
}

macro_rules! Arm {
    () => {
        deps!();
        # [doc = " Represents a single arm of a `match` expression, e.g."] # [doc = " `<pat> (if <guard>) => <body>`."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct Arm < 'hir > { # [stable_hasher (ignore)] pub hir_id : HirId , pub span : Span , # [doc = " If this pattern and the optional guard matches, then `body` is evaluated."] pub pat : & 'hir Pat < 'hir > , # [doc = " Optional guard clause."] pub guard : Option < & 'hir Expr < 'hir > > , # [doc = " The expression the arm evaluates to if this arm matches."] pub body : & 'hir Expr < 'hir > , }
    };
}

Arm!();