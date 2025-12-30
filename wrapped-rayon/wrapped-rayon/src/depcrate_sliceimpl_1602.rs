// Generated macro for impl_1602 (impl)
macro_rules! Depcrate_sliceimpl_1602 {
() => {
// Module: crate::slice
// Provides: {"impl_1602"}
// Dependencies: {}
impl < T : Debug , P > Debug for SplitInclusive < '_ , T , P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SplitInclusive") . field ("slice" , & self . slice) . finish () } }
};
}
