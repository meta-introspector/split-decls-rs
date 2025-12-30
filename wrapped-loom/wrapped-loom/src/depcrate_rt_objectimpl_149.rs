// Generated macro for impl_149 (impl)
macro_rules! Depcrate_rt_objectimpl_149 {
() => {
// Module: crate::rt::object
// Provides: {"impl_149"}
// Dependencies: {}
impl < T > fmt :: Debug for Ref < T > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use std :: any :: type_name ; write ! (fmt , "Ref<{}>({})" , type_name ::< T > () , self . index) } }
};
}
