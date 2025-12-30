// Generated macro for impl_53 (impl)
macro_rules! Depcrate_checkedimpl_53 {
() => {
// Module: crate::checked
// Provides: {"impl_53"}
// Dependencies: {}
impl < T > Mul < & Self > for Checked < T > where T : CheckedMul + ConditionallySelectable + Default , { type Output = Checked < T > ; # [inline] fn mul (self , rhs : & Self) -> Self :: Output { Checked (self . 0 . and_then (| lhs | rhs . 0 . and_then (| rhs | lhs . checked_mul (& rhs))) ,) } }
};
}
