// Generated macro for impl_1078 (impl)
macro_rules! Depcrate_recovery_bandwidthimpl_1078 {
() => {
// Module: crate::recovery::bandwidth
// Provides: {"impl_1078"}
// Dependencies: {}
impl std :: fmt :: Debug for Bandwidth { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { match self . bits_per_second { x if x < 1_000_000 => write ! (f , "{:.2} Kbps" , x as f64 / 1_000.) , x if x < 1_000_000_000 => { write ! (f , "{:.2} Mbps" , x as f64 / 1_000_000.) } , x => write ! (f , "{:.2} Gbps" , x as f64 / 1_000_000_000.) , } } }
};
}
