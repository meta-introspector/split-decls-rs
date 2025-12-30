// Generated macro for impl_1014 (impl)
macro_rules! Depcrate_iter_repeatimpl_1014 {
() => {
// Module: crate::iter::repeat
// Provides: {"impl_1014"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for RepeatN < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut dbg = f . debug_struct ("RepeatN") ; if let RepeatNProducer :: Repeats (element , count) = & self . inner { dbg . field ("count" , & count . get ()) . field ("element" , element) . finish () } else { dbg . field ("count" , & 0usize) . finish_non_exhaustive () } } }
};
}
