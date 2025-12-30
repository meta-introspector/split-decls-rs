// Generated macro for invert_op (function)
macro_rules! Depcrate_implicit_saturating_addinvert_op {
() => {
// Module: crate::implicit_saturating_add
// Provides: {"invert_op"}
// Dependencies: {}
fn invert_op (op : BinOpKind) -> Option < BinOpKind > { use rustc_hir :: BinOpKind :: { Ge , Gt , Le , Lt , Ne } ; match op { Lt => Some (Gt) , Le => Some (Ge) , Ne => Some (Ne) , Ge => Some (Le) , Gt => Some (Lt) , _ => None , } }
};
}
