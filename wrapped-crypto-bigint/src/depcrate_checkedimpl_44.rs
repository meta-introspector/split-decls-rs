// Generated macro for impl_44 (impl)
macro_rules! Depcrate_checkedimpl_44 {
() => {
// Module: crate::checked
// Provides: {"impl_44"}
// Dependencies: {}
impl < T > Add < Self > for Checked < T > where T : CheckedAdd + ConditionallySelectable + Default , { type Output = Checked < T > ; # [inline] fn add (self , rhs : Self) -> Self :: Output { Checked (self . 0 . and_then (| lhs | rhs . 0 . and_then (| rhs | lhs . checked_add (& rhs))) ,) } }
};
}
