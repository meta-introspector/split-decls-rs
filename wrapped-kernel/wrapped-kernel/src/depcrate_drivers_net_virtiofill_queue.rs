// Generated macro for fill_queue (function)
macro_rules! Depcrate_drivers_net_virtiofill_queue {
() => {
// Module: crate::drivers::net::virtio
// Provides: {"fill_queue"}
// Dependencies: {}
fn fill_queue (vq : & mut VirtQueue , num_bufs : u16 , buf_size : u32) { for _ in 0 .. num_bufs { let buff_tkn = buffer_token_from_hdr (Box :: < Hdr , _ > :: new_uninit_in (DeviceAlloc) , buf_size) ; if let Err (err) = vq . dispatch (buff_tkn , false , BufferType :: Direct) { error ! ("{err:#?}") ; break ; } } }
};
}
