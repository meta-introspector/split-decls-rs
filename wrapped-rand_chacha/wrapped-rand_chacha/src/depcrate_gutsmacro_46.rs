// Generated macro for macro_46 (macro)
macro_rules! Depcrate_gutsmacro_46 {
() => {
// Module: crate::guts
// Provides: {"macro_46"}
// Dependencies: {}
dispatch_light128 ! (m , Mach , { fn set_stream_param (state : & mut ChaCha , param : u32 , value : u64) { let d : Mach :: u32x4 = m . unpack (state . d) ; state . d = d . insert ((value >> 32) as u32 , (param << 1) | 1) . insert (value as u32 , param << 1) . into () ; } }) ;
};
}
