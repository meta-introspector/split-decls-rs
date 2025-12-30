// Generated macro for is_block_like (function)
macro_rules! Depcrateis_block_like {
() => {
// Module: crate
// Provides: {"is_block_like"}
// Dependencies: {}
# [doc = " Returns true if the given `expr` is a block or resembled as a block,"] # [doc = " such as `if`, `loop`, `match` expressions etc."] pub fn is_block_like (expr : & Expr < '_ >) -> bool { matches ! (expr . kind , ExprKind :: Block (..) | ExprKind :: ConstBlock (..) | ExprKind :: If (..) | ExprKind :: Loop (..) | ExprKind :: Match (..)) }
};
}
