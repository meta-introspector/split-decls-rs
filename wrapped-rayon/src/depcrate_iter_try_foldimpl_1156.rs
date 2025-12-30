// Generated macro for impl_1156 (impl)
macro_rules! Depcrate_iter_try_foldimpl_1156 {
() => {
// Module: crate::iter::try_fold
// Provides: {"impl_1156"}
// Dependencies: {}
impl < I , U , F > Debug for TryFoldWith < I , U , F > where I : Debug , U : Try < Output : Debug > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TryFoldWith") . field ("base" , & self . base) . field ("item" , & self . item) . finish () } }
};
}
