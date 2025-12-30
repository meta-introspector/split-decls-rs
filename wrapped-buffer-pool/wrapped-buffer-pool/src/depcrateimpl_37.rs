// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'a , const S : usize , T : Default + Extend < & 'a u8 > + Reuse > Pool < S , T > { # [doc = " Get a value from the pool and extend it with the provided slice."] pub fn with_slice (& 'static self , v : & 'a [u8]) -> Pooled < T > { let mut buf = self . get () ; buf . deref_mut () . extend (v) ; buf } }
};
}
