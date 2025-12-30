// Generated macro for impl_158 (impl)
macro_rules! Depcrate_utilsimpl_158 {
() => {
// Module: crate::utils
// Provides: {"impl_158"}
// Dependencies: {}
impl < T , const N : usize > MakeMaybeUninit < T , N > { pub (crate) const VALUE : MaybeUninit < T > = MaybeUninit :: uninit () ; pub (crate) const ARRAY : [MaybeUninit < T > ; N] = [Self :: VALUE ; N] ; }
};
}
