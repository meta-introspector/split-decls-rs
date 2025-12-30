// Generated macro for impl_142 (impl)
macro_rules! Depcrate_com_objectimpl_142 {
() => {
// Module: crate::com_object
// Provides: {"impl_142"}
// Dependencies: {}
impl < T : ComObjectInner + core :: hash :: Hash > core :: hash :: Hash for ComObject < T > { fn hash < H : core :: hash :: Hasher > (& self , state : & mut H) { self . get () . hash (state) ; } }
};
}
