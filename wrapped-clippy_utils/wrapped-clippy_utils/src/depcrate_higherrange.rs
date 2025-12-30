// Generated macro for Range (struct)
macro_rules! Depcrate_higherRange {
() => {
// Module: crate::higher
// Provides: {"Range"}
// Dependencies: {}
# [doc = " Represent a range akin to `ast::ExprKind::Range`."] # [derive (Debug , Copy , Clone)] pub struct Range < 'a > { # [doc = " The lower bound of the range, or `None` for ranges such as `..X`."] pub start : Option < & 'a Expr < 'a > > , # [doc = " The upper bound of the range, or `None` for ranges such as `X..`."] pub end : Option < & 'a Expr < 'a > > , # [doc = " Whether the interval is open or closed."] pub limits : ast :: RangeLimits , pub span : Span , }
};
}
