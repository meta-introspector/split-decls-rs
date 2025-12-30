// Generated macro for impl_59 (impl)
macro_rules! Depcrate_checkedimpl_59 {
() => {
// Module: crate::checked
// Provides: {"impl_59"}
// Dependencies: {}
impl < T > Div < & Checked < T > > for & Checked < T > where T : CheckedDiv + ConditionallySelectable + Default , { type Output = Checked < T > ; # [inline] fn div (self , rhs : & Checked < T >) -> Self :: Output { Checked (self . 0 . and_then (| lhs | rhs . 0 . and_then (| rhs | lhs . checked_div (& rhs))) ,) } }
};
}
