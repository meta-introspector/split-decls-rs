// Generated macro for autodetect (module)
macro_rules! Depcrate_compressorautodetect {
() => {
// Module: crate::compressor
// Provides: {"autodetect"}
// Dependencies: {}
# [cfg (feature = "std")] mod autodetect { use super :: * ; type Tf < T > = unsafe fn (cv : & mut T , data : * const u8) ; type Of < T > = unsafe fn (cv : & mut T) ; type Init < T > = unsafe fn (cv : T) -> T ; macro_rules ! dispatch { ($ fn : ident , $ ty : ty) => { fn dispatch_init () -> $ ty { if is_x86_feature_detected ! ("aes") { aes ::$ fn } else if is_x86_feature_detected ! ("ssse3") { ssse3 ::$ fn } else if is_x86_feature_detected ! ("sse2") { sse2 ::$ fn } else { panic ! ("groestl_aesni requires at least sse2 (not detected)") } } lazy_static ! { static ref IMPL : $ ty = { dispatch_init () } ; } } ; } # [inline] pub fn tf512 (cv : & mut X4 , data : & GenericArray < u8 , U64 >) { dispatch ! (tf512 , Tf < X4 >) ; unsafe { IMPL (cv , data . as_ptr ()) } } # [inline] pub fn of512 (cv : & mut X4) { dispatch ! (of512 , Of < X4 >) ; unsafe { IMPL (cv) } } # [inline] pub fn init512 (cv : X4) -> X4 { dispatch ! (init512 , Init < X4 >) ; unsafe { IMPL (cv) } } # [inline] pub fn tf1024 (cv : & mut X8 , data : & GenericArray < u8 , U128 >) { dispatch ! (tf1024 , Tf < X8 >) ; unsafe { IMPL (cv , data . as_ptr ()) } } # [inline] pub fn of1024 (cv : & mut X8) { dispatch ! (of1024 , Of < X8 >) ; unsafe { IMPL (cv) } } # [inline] pub fn init1024 (cv : X8) -> X8 { dispatch ! (init1024 , Init < X8 >) ; unsafe { IMPL (cv) } } }
};
}
