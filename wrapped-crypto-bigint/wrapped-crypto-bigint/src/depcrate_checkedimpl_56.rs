// Generated macro for impl_56 (impl)
macro_rules! Depcrate_checkedimpl_56 {
() => {
// Module: crate::checked
// Provides: {"impl_56"}
// Dependencies: {}
impl < T > Div < Self > for Checked < T > where T : CheckedDiv + ConditionallySelectable + Default , { type Output = Checked < T > ; # [inline] fn div (self , rhs : Self) -> Self :: Output { Checked (self . 0 . and_then (| lhs | rhs . 0 . and_then (| rhs | lhs . checked_div (& rhs))) ,) } }
};
}
