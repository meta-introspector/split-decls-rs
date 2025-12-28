macro_rules! RemoveUnnecessaryElse {
    () => {
        # [derive (Debug)] pub struct RemoveUnnecessaryElse { pub if_expr : InFile < AstPtr < ast :: IfExpr > > , }
    };
}

RemoveUnnecessaryElse!()