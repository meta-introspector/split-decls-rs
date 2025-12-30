// Generated macro for get_shift_mask (function)
macro_rules! Depcrate_arm_linuxget_shift_mask {
() => {
// Module: crate::arm_linux
// Provides: {"get_shift_mask"}
// Dependencies: {}
fn get_shift_mask < T > (ptr : * mut T) -> (u32 , u32) { let mask = match mem :: size_of :: < T > () { 1 => 0xff , 2 => 0xffff , 4 => 0xffffffff , _ => unreachable ! () , } ; let endian_adjust = if cfg ! (target_endian = "little") { 0 } else { 4 - mem :: size_of :: < T > () as u32 } ; let ptr_mask = 3 & (4 - mem :: size_of :: < T > ()) ; let shift = ((ptr as usize & ptr_mask) as u32 ^ endian_adjust) * 8 ; (shift , mask) }
};
}
