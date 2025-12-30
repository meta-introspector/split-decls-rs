// Generated macro for impl_55 (impl)
macro_rules! Depcrate_checkedimpl_55 {
() => {
// Module: crate::checked
// Provides: {"impl_55"}
// Dependencies: {}
impl < T > Mul < & Checked < T > > for & Checked < T > where T : CheckedMul + ConditionallySelectable + Default , { type Output = Checked < T > ; # [inline] fn mul (self , rhs : & Checked < T >) -> Self :: Output { Checked (self . 0 . and_then (| lhs | rhs . 0 . and_then (| rhs | lhs . checked_mul (& rhs))) ,) } }
};
}
