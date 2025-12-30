// Generated macro for impl_46 (impl)
macro_rules! Depcrate_checkedimpl_46 {
() => {
// Module: crate::checked
// Provides: {"impl_46"}
// Dependencies: {}
impl < T > Add < Checked < T > > for & Checked < T > where T : CheckedAdd + ConditionallySelectable + Default , { type Output = Checked < T > ; # [inline] fn add (self , rhs : Checked < T >) -> Self :: Output { Checked (self . 0 . and_then (| lhs | rhs . 0 . and_then (| rhs | lhs . checked_add (& rhs))) ,) } }
};
}
