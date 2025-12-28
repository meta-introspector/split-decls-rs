macro_rules! deps {
    () => {
        LetStmt!();
        Pat!();
        Ty!();
        Expr!();
    };
}

macro_rules! LetExpr {
    () => {
        deps!();
        # [doc = " Represents a `let <pat>[: <ty>] = <expr>` expression (not a [`LetStmt`]), occurring in an `if-let`"] # [doc = " or `let-else`, evaluating to a boolean. Typically the pattern is refutable."] # [doc = ""] # [doc = " In an `if let`, imagine it as `if (let <pat> = <expr>) { ... }`; in a let-else, it is part of"] # [doc = " the desugaring to if-let. Only let-else supports the type annotation at present."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct LetExpr < 'hir > { pub span : Span , pub pat : & 'hir Pat < 'hir > , pub ty : Option < & 'hir Ty < 'hir > > , pub init : & 'hir Expr < 'hir > , # [doc = " `Recovered::Yes` when this let expressions is not in a syntactically valid location."] # [doc = " Used to prevent building MIR in such situations."] pub recovered : ast :: Recovered , }
    };
}

LetExpr!();