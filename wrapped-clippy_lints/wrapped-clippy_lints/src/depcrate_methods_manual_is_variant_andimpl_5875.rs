// Generated macro for impl_5875 (impl)
macro_rules! Depcrate_methods_manual_is_variant_andimpl_5875 {
() => {
// Module: crate::methods::manual_is_variant_and
// Provides: {"impl_5875"}
// Dependencies: {}
impl TryFrom < BinOpKind > for Op { type Error = () ; fn try_from (op : BinOpKind) -> Result < Self , Self :: Error > { match op { BinOpKind :: Eq => Ok (Self :: Eq) , BinOpKind :: Ne => Ok (Self :: Ne) , _ => Err (()) , } } }
};
}
