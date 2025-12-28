macro_rules! deps {
    () => {
        Item!();
        ItemId!();
        LetStmt!();
        Expr!();
    };
}

macro_rules! StmtKind {
    () => {
        deps!();
        # [doc = " The contents of a statement."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum StmtKind < 'hir > { # [doc = " A local (`let`) binding."] Let (& 'hir LetStmt < 'hir >) , # [doc = " An item binding."] Item (ItemId) , # [doc = " An expression without a trailing semi-colon (must have unit type)."] Expr (& 'hir Expr < 'hir >) , # [doc = " An expression with a trailing semi-colon (may have any type)."] Semi (& 'hir Expr < 'hir >) , }
    };
}

StmtKind!();