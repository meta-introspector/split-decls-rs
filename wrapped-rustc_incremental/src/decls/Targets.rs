macro_rules! Targets {
    () => {
        type Targets = Vec < (Span , Symbol , hir :: HirId , DepNode) > ;
    };
}

Targets!();