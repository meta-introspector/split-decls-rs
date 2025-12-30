// Generated macro for impl_47 (impl)
macro_rules! Depcrate_fingerprintimpl_47 {
() => {
// Module: crate::fingerprint
// Provides: {"impl_47"}
// Dependencies: {}
impl Hash for Fingerprint { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { state . write_fingerprint (self) ; } }
};
}
