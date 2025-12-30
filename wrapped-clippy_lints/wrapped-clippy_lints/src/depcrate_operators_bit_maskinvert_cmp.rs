// Generated macro for invert_cmp (function)
macro_rules! Depcrate_operators_bit_maskinvert_cmp {
() => {
// Module: crate::operators::bit_mask
// Provides: {"invert_cmp"}
// Dependencies: {}
# [must_use] fn invert_cmp (cmp : BinOpKind) -> BinOpKind { match cmp { BinOpKind :: Eq => BinOpKind :: Eq , BinOpKind :: Ne => BinOpKind :: Ne , BinOpKind :: Lt => BinOpKind :: Gt , BinOpKind :: Gt => BinOpKind :: Lt , BinOpKind :: Le => BinOpKind :: Ge , BinOpKind :: Ge => BinOpKind :: Le , _ => BinOpKind :: Or , } }
};
}
