// Generated macro for impl_145 (impl)
macro_rules! Depcrate_com_objectimpl_145 {
() => {
// Module: crate::com_object
// Provides: {"impl_145"}
// Dependencies: {}
impl < T : ComObjectInner + PartialEq > PartialEq for ComObject < T > { fn eq (& self , other : & Self) -> bool { let inner_self : & T = self . get () ; let other_self : & T = other . get () ; inner_self == other_self } }
};
}
