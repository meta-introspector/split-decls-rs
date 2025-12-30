// Generated macro for Buffer (struct)
macro_rules! Depcrate_bridge_bufferBuffer {
() => {
// Module: crate::bridge::buffer
// Provides: {"Buffer"}
// Dependencies: {}
# [repr (C)] pub struct Buffer { data : * mut u8 , len : usize , capacity : usize , reserve : extern "C" fn (Buffer , usize) -> Buffer , drop : extern "C" fn (Buffer) , }
};
}
