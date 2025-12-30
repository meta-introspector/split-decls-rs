// Generated macro for impl_56 (impl)
macro_rules! Depcrate_dequeimpl_56 {
() => {
// Module: crate::deque
// Provides: {"impl_56"}
// Dependencies: {}
impl < T : fmt :: Debug , S : VecStorage < T > + ? Sized > fmt :: Debug for DequeInner < T , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self) . finish () } }
};
}
