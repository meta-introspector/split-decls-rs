// Generated macro for eq_defaultness (function)
macro_rules! Depcrate_ast_utilseq_defaultness {
() => {
// Module: crate::ast_utils
// Provides: {"eq_defaultness"}
// Dependencies: {}
pub fn eq_defaultness (l : Defaultness , r : Defaultness) -> bool { matches ! ((l , r) , (Defaultness :: Final , Defaultness :: Final) | (Defaultness :: Default (_) , Defaultness :: Default (_))) }
};
}
