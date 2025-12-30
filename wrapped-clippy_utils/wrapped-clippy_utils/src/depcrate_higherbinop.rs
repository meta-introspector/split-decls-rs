// Generated macro for binop (function)
macro_rules! Depcrate_higherbinop {
() => {
// Module: crate::higher
// Provides: {"binop"}
// Dependencies: {}
# [doc = " Converts a `hir` binary operator to the corresponding `ast` type."] # [must_use] pub fn binop (op : hir :: BinOpKind) -> ast :: BinOpKind { match op { hir :: BinOpKind :: Eq => ast :: BinOpKind :: Eq , hir :: BinOpKind :: Ge => ast :: BinOpKind :: Ge , hir :: BinOpKind :: Gt => ast :: BinOpKind :: Gt , hir :: BinOpKind :: Le => ast :: BinOpKind :: Le , hir :: BinOpKind :: Lt => ast :: BinOpKind :: Lt , hir :: BinOpKind :: Ne => ast :: BinOpKind :: Ne , hir :: BinOpKind :: Or => ast :: BinOpKind :: Or , hir :: BinOpKind :: Add => ast :: BinOpKind :: Add , hir :: BinOpKind :: And => ast :: BinOpKind :: And , hir :: BinOpKind :: BitAnd => ast :: BinOpKind :: BitAnd , hir :: BinOpKind :: BitOr => ast :: BinOpKind :: BitOr , hir :: BinOpKind :: BitXor => ast :: BinOpKind :: BitXor , hir :: BinOpKind :: Div => ast :: BinOpKind :: Div , hir :: BinOpKind :: Mul => ast :: BinOpKind :: Mul , hir :: BinOpKind :: Rem => ast :: BinOpKind :: Rem , hir :: BinOpKind :: Shl => ast :: BinOpKind :: Shl , hir :: BinOpKind :: Shr => ast :: BinOpKind :: Shr , hir :: BinOpKind :: Sub => ast :: BinOpKind :: Sub , } }
};
}
