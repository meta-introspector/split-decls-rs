// Generated macro for kind_is_cmp (function)
macro_rules! Depcrate_comparison_chainkind_is_cmp {
() => {
// Module: crate::comparison_chain
// Provides: {"kind_is_cmp"}
// Dependencies: {}
fn kind_is_cmp (kind : BinOpKind) -> bool { matches ! (kind , BinOpKind :: Lt | BinOpKind :: Gt | BinOpKind :: Eq) }
};
}
