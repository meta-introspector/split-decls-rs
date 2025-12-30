// Generated macro for IfLetOrMatch (enum)
macro_rules! Depcrate_higherIfLetOrMatch {
() => {
// Module: crate::higher
// Provides: {"IfLetOrMatch"}
// Dependencies: {}
# [doc = " An `if let` or `match` expression. Useful for lints that trigger on one or the other."] # [derive (Debug)] pub enum IfLetOrMatch < 'hir > { # [doc = " Any `match` expression"] Match (& 'hir Expr < 'hir > , & 'hir [Arm < 'hir >] , MatchSource) , # [doc = " scrutinee, pattern, then block, else block"] IfLet (& 'hir Expr < 'hir > , & 'hir Pat < 'hir > , & 'hir Expr < 'hir > , Option < & 'hir Expr < 'hir > > , # [doc = " `if let PAT = EXPR`"] # [doc = "     ^^^^^^^^^^^^^^"] Span ,) , }
};
}
