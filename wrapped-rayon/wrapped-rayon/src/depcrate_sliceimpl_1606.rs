// Generated macro for impl_1606 (impl)
macro_rules! Depcrate_sliceimpl_1606 {
() => {
// Module: crate::slice
// Provides: {"impl_1606"}
// Dependencies: {}
impl < T : Debug , P > Debug for SplitMut < '_ , T , P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SplitMut") . field ("slice" , & self . slice) . finish () } }
};
}
