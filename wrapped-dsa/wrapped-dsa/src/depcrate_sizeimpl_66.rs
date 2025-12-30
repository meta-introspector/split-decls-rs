// Generated macro for impl_66 (impl)
macro_rules! Depcrate_sizeimpl_66 {
() => {
// Module: crate::size
// Provides: {"impl_66"}
// Dependencies: {}
impl KeySize { pub (crate) fn l_aligned (& self) -> u32 { self . l . div_ceil (Limb :: BITS) * Limb :: BITS } pub (crate) fn n_aligned (& self) -> u32 { self . n . div_ceil (Limb :: BITS) * Limb :: BITS } pub (crate) fn matches (& self , l : u32 , n : u32) -> bool { l == self . l_aligned () && n == self . n_aligned () } }
};
}
