// Generated macro for impl_1147 (impl)
macro_rules! Depcrate_iter_try_foldimpl_1147 {
() => {
// Module: crate::iter::try_fold
// Provides: {"impl_1147"}
// Dependencies: {}
impl < U , I : ParallelIterator + Debug , ID , F > Debug for TryFold < I , U , ID , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TryFold") . field ("base" , & self . base) . finish () } }
};
}
