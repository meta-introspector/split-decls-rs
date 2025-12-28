macro_rules! deps {
    () => {
        StmtKind!();
    };
}

macro_rules! Stmt {
    () => {
        deps!();
        # [doc = " A statement."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct Stmt < 'hir > { # [stable_hasher (ignore)] pub hir_id : HirId , pub kind : StmtKind < 'hir > , pub span : Span , }
    };
}

Stmt!();