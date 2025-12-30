// Generated macro for impl_30 (impl)
macro_rules! Depcrate_poly1305impl_30 {
() => {
// Module: crate::poly1305
// Provides: {"impl_30"}
// Dependencies: {}
impl Drop for Poly1305 { fn drop (& mut self) { self . h = [0 ; 5] ; self . r = [0 ; 5] ; self . pad = [0 ; 4] ; core :: sync :: atomic :: fence (core :: sync :: atomic :: Ordering :: Release) ; } }
};
}
