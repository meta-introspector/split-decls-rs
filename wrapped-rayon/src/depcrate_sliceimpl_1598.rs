// Generated macro for impl_1598 (impl)
macro_rules! Depcrate_sliceimpl_1598 {
() => {
// Module: crate::slice
// Provides: {"impl_1598"}
// Dependencies: {}
impl < T : Debug , P > Debug for Split < '_ , T , P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Split") . field ("slice" , & self . slice) . finish () } }
};
}
