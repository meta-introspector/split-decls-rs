// Generated macro for impl_50 (impl)
macro_rules! Depcrate_checkedimpl_50 {
() => {
// Module: crate::checked
// Provides: {"impl_50"}
// Dependencies: {}
impl < T > Sub < Checked < T > > for & Checked < T > where T : CheckedSub + ConditionallySelectable + Default , { type Output = Checked < T > ; # [inline] fn sub (self , rhs : Checked < T >) -> Self :: Output { Checked (self . 0 . and_then (| lhs | rhs . 0 . and_then (| rhs | lhs . checked_sub (& rhs))) ,) } }
};
}
