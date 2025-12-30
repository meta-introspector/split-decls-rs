// Generated macro for impl_79 (impl)
macro_rules! Depcrate_integerimpl_79 {
() => {
// Module: crate::integer
// Provides: {"impl_79"}
// Dependencies: {}
impl TryFrom < Decimal > for FixedInteger { type Error = LimitError ; fn try_from (signed_fd : Decimal) -> Result < Self , Self :: Error > { if signed_fd . absolute . magnitude_range () . start () != & 0 { Err (LimitError) } else { Ok (FixedInteger (signed_fd)) } } }
};
}
