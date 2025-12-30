// Generated macro for impl_49 (impl)
macro_rules! Depcrate_checkedimpl_49 {
() => {
// Module: crate::checked
// Provides: {"impl_49"}
// Dependencies: {}
impl < T > Sub < & Self > for Checked < T > where T : CheckedSub + ConditionallySelectable + Default , { type Output = Checked < T > ; # [inline] fn sub (self , rhs : & Self) -> Self :: Output { Checked (self . 0 . and_then (| lhs | rhs . 0 . and_then (| rhs | lhs . checked_sub (& rhs))) ,) } }
};
}
