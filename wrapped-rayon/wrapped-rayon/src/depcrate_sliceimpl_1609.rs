// Generated macro for impl_1609 (impl)
macro_rules! Depcrate_sliceimpl_1609 {
() => {
// Module: crate::slice
// Provides: {"impl_1609"}
// Dependencies: {}
impl < T : Debug , P > Debug for SplitInclusiveMut < '_ , T , P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SplitInclusiveMut") . field ("slice" , & self . slice) . finish () } }
};
}
