// Generated macro for ExpressionStoreDiagnostics (enum)
macro_rules! Depcrate_expr_storeExpressionStoreDiagnostics {
() => {
// Module: crate::expr_store
// Provides: {"ExpressionStoreDiagnostics"}
// Dependencies: {}
# [derive (Debug , Eq , PartialEq)] pub enum ExpressionStoreDiagnostics { InactiveCode { node : InFile < SyntaxNodePtr > , cfg : CfgExpr , opts : CfgOptions } , UnresolvedMacroCall { node : InFile < MacroCallPtr > , path : ModPath } , UnreachableLabel { node : InFile < AstPtr < ast :: Lifetime > > , name : Name } , AwaitOutsideOfAsync { node : InFile < AstPtr < ast :: AwaitExpr > > , location : String } , UndeclaredLabel { node : InFile < AstPtr < ast :: Lifetime > > , name : Name } , }
};
}
