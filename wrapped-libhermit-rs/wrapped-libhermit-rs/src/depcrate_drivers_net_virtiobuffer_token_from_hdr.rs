// Generated macro for buffer_token_from_hdr (function)
macro_rules! Depcrate_drivers_net_virtiobuffer_token_from_hdr {
() => {
// Module: crate::drivers::net::virtio
// Provides: {"buffer_token_from_hdr"}
// Dependencies: {}
fn buffer_token_from_hdr (hdr : Box < MaybeUninit < Hdr > , DeviceAlloc > , buf_size : u32 ,) -> AvailBufferToken { AvailBufferToken :: new (SmallVec :: new () , { SmallVec :: from_buf ([BufferElem :: Sized (hdr) , BufferElem :: Vector (Vec :: with_capacity_in (buf_size . try_into () . unwrap () , DeviceAlloc ,)) ,]) }) . unwrap () }
};
}
