macro_rules! deps {
    () => {
        DotAccessExprCtx!();
        DotAccessKind!();
    };
}

macro_rules! DotAccess {
    () => {
        deps!();
        # [doc = " Information about the field or method access we are completing."] # [derive (Debug)] pub (crate) struct DotAccess < 'db > { pub (crate) receiver : Option < ast :: Expr > , pub (crate) receiver_ty : Option < TypeInfo < 'db > > , pub (crate) kind : DotAccessKind , pub (crate) ctx : DotAccessExprCtx , }
    };
}

DotAccess!();