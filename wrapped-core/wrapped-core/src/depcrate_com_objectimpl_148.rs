// Generated macro for impl_148 (impl)
macro_rules! Depcrate_com_objectimpl_148 {
() => {
// Module: crate::com_object
// Provides: {"impl_148"}
// Dependencies: {}
impl < T : ComObjectInner + Ord > Ord for ComObject < T > { fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { let inner_self : & T = self . get () ; let other_self : & T = other . get () ; < T as Ord > :: cmp (inner_self , other_self) } }
};
}
