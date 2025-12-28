macro_rules! deps {
    () => {
        MacroCallPtr!();
    };
}

macro_rules! ExpressionStoreDiagnostics {
    () => {
        deps!();
        # [derive (Debug , Eq , PartialEq)] pub enum ExpressionStoreDiagnostics { InactiveCode { node : InFile < SyntaxNodePtr > , cfg : CfgExpr , opts : CfgOptions } , UnresolvedMacroCall { node : InFile < MacroCallPtr > , path : ModPath } , UnreachableLabel { node : InFile < AstPtr < ast :: Lifetime > > , name : Name } , AwaitOutsideOfAsync { node : InFile < AstPtr < ast :: AwaitExpr > > , location : String } , UndeclaredLabel { node : InFile < AstPtr < ast :: Lifetime > > , name : Name } , }
    };
}

ExpressionStoreDiagnostics!()