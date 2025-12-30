// Generated macro for impl_147 (impl)
macro_rules! Depcrate_com_objectimpl_147 {
() => {
// Module: crate::com_object
// Provides: {"impl_147"}
// Dependencies: {}
impl < T : ComObjectInner + PartialOrd > PartialOrd for ComObject < T > { fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { let inner_self : & T = self . get () ; let other_self : & T = other . get () ; < T as PartialOrd > :: partial_cmp (inner_self , other_self) } }
};
}
