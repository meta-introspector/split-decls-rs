// Generated macro for impl_150 (impl)
macro_rules! Depcrate_com_objectimpl_150 {
() => {
// Module: crate::com_object
// Provides: {"impl_150"}
// Dependencies: {}
impl < T : ComObjectInner + core :: fmt :: Display > core :: fmt :: Display for ComObject < T > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { < T as core :: fmt :: Display > :: fmt (self . get () , f) } }
};
}
