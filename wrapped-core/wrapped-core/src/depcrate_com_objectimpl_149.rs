// Generated macro for impl_149 (impl)
macro_rules! Depcrate_com_objectimpl_149 {
() => {
// Module: crate::com_object
// Provides: {"impl_149"}
// Dependencies: {}
impl < T : ComObjectInner + core :: fmt :: Debug > core :: fmt :: Debug for ComObject < T > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { < T as core :: fmt :: Debug > :: fmt (self . get () , f) } }
};
}
