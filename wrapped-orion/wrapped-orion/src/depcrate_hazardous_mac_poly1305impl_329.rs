// Generated macro for impl_329 (impl)
macro_rules! Depcrate_hazardous_mac_poly1305impl_329 {
() => {
// Module: crate::hazardous::mac::poly1305
// Provides: {"impl_329"}
// Dependencies: {}
impl Drop for Poly1305 { fn drop (& mut self) { use zeroize :: Zeroize ; self . a . 0 . zeroize () ; self . r . 0 . zeroize () ; self . s . zeroize () ; self . buffer . zeroize () ; } }
};
}
