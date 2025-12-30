// Generated macro for impl_647 (impl)
macro_rules! Depcrate_drivers_virtio_virtqueueimpl_647 {
() => {
// Module: crate::drivers::virtio::virtqueue
// Provides: {"impl_647"}
// Dependencies: {}
impl AvailBufferToken { # [doc = " **Parameters**"] # [doc = " * send: The slices that will make up the elements of the driver-writable buffer."] # [doc = " * recv: The slices that will make up the elements of the device-writable buffer."] # [doc = ""] # [doc = " **Reasons for Failure:**"] # [doc = " * Both `send` and `recv` are empty, which is not allowed by Virtio."] # [doc = ""] # [doc = " * If one wants to have a structure in the style of:"] # [doc = " ```"] # [doc = " struct send_recv_struct {"] # [doc = "     // send_part: ..."] # [doc = "     // recv_part: ..."] # [doc = " }"] # [doc = " ```"] # [doc = " they must split the structure after the send part and provide the respective part via the send argument and the respective other"] # [doc = " part via the recv argument."] pub fn new (send_buff : SmallVec < [BufferElem ; 2] > , recv_buff : SmallVec < [BufferElem ; 2] > ,) -> Result < Self , VirtqError > { if send_buff . is_empty () && recv_buff . is_empty () { return Err (VirtqError :: BufferNotSpecified) ; } Ok (Self { send_buff , recv_buff , }) } }
};
}
