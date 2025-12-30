// Generated macro for impl_48 (impl)
macro_rules! Depcrate_checkedimpl_48 {
() => {
// Module: crate::checked
// Provides: {"impl_48"}
// Dependencies: {}
impl < T > Sub < Self > for Checked < T > where T : CheckedSub + ConditionallySelectable + Default , { type Output = Checked < T > ; # [inline] fn sub (self , rhs : Self) -> Self :: Output { Checked (self . 0 . and_then (| lhs | rhs . 0 . and_then (| rhs | lhs . checked_sub (& rhs))) ,) } }
};
}
