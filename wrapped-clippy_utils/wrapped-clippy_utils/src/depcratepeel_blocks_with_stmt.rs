// Generated macro for peel_blocks_with_stmt (function)
macro_rules! Depcratepeel_blocks_with_stmt {
() => {
// Module: crate
// Provides: {"peel_blocks_with_stmt"}
// Dependencies: {}
# [doc = " Removes blocks around an expression, only if the block contains just one expression"] # [doc = " or just one expression statement with a semicolon. Unsafe blocks are not removed."] # [doc = ""] # [doc = " Examples:"] # [doc = "  * `{}`               -> `{}`"] # [doc = "  * `{ x }`            -> `x`"] # [doc = "  * `{ x; }`           -> `x`"] # [doc = "  * `{{ x; }}`         -> `x`"] # [doc = "  * `{ x; y }`         -> `{ x; y }`"] # [doc = "  * `{ unsafe { x } }` -> `unsafe { x }`"] pub fn peel_blocks_with_stmt < 'a > (mut expr : & 'a Expr < 'a >) -> & 'a Expr < 'a > { while let ExprKind :: Block (Block { stmts : [] , expr : Some (inner) , rules : BlockCheckMode :: DefaultBlock , .. } | Block { stmts : [Stmt { kind : StmtKind :: Expr (inner) | StmtKind :: Semi (inner) , .. } ,] , expr : None , rules : BlockCheckMode :: DefaultBlock , .. } , _ ,) = expr . kind { expr = inner ; } expr }
};
}
