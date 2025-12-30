// Generated macro for macro_47 (macro)
macro_rules! Depcrate_gutsmacro_47 {
() => {
// Module: crate::guts
// Provides: {"macro_47"}
// Dependencies: {}
dispatch_light128 ! (m , Mach , { fn get_stream_param (state : & ChaCha , param : u32) -> u64 { let d : Mach :: u32x4 = m . unpack (state . d) ; ((d . extract ((param << 1) | 1) as u64) << 32) | d . extract (param << 1) as u64 } }) ;
};
}
