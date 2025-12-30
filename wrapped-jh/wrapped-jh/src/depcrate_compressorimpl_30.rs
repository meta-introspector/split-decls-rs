// Generated macro for impl_30 (impl)
macro_rules! Depcrate_compressorimpl_30 {
() => {
// Module: crate::compressor
// Provides: {"impl_30"}
// Dependencies: {}
impl Compressor { # [inline] pub (crate) fn new (bytes : [u8 ; 128]) -> Self { Compressor { bytes } } # [inline] # [allow (unexpected_cfgs)] pub (crate) fn update (& mut self , data : & Array < u8 , U64 >) { simd :: dispatch ! (mach , M , { fn f8 (state : & mut [vec128_storage ; 8] , data : * const u8) { f8_impl (mach , state , data) ; } }) ; f8 (unsafe { & mut self . cv } , data . as_ptr ()) ; } # [inline] pub (crate) fn finalize (& self) -> & [u8 ; 128] { unsafe { & self . bytes } } }
};
}
