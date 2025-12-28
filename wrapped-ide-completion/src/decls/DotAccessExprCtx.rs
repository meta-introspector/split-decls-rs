macro_rules! deps {
    () => {
        BreakableKind!();
    };
}

macro_rules! DotAccessExprCtx {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub (crate) struct DotAccessExprCtx { pub (crate) in_block_expr : bool , pub (crate) in_breakable : Option < BreakableKind > , }
    };
}

DotAccessExprCtx!();