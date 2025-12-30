// Generated macro for impl_30 (impl)
macro_rules! Depcrate_cryptoimpl_30 {
() => {
// Module: crate::crypto
// Provides: {"impl_30"}
// Dependencies: {}
impl < Shake : ExtendableOutput + Default + Clone > ShakeState < Shake > { pub (crate) fn updatable (& mut self) -> & mut Shake { match self { Self :: Absorbing (sponge) => sponge , Self :: Squeezing (_) => unreachable ! () , } } pub (crate) fn absorb (mut self , input : & [u8]) -> Self { match & mut self { Self :: Absorbing (sponge) => sponge . update (input) , Self :: Squeezing (_) => unreachable ! () , } self } pub (crate) fn squeeze (& mut self , output : & mut [u8]) -> & mut Self { match self { Self :: Absorbing (sponge) => { let mut reader = sponge . clone () . finalize_xof () ; reader . read (output) ; * self = Self :: Squeezing (reader) ; } Self :: Squeezing (reader) => { reader . read (output . as_mut ()) ; } } self } pub (crate) fn squeeze_new < N : ArraySize > (& mut self) -> Array < u8 , N > { let mut v = Array :: default () ; self . squeeze (& mut v) ; v } }
};
}
