// Generated macro for fill_queue (function)
macro_rules! Depcrate_drivers_vsockfill_queue {
() => {
// Module: crate::drivers::vsock
// Provides: {"fill_queue"}
// Dependencies: {}
fn fill_queue (vq : & mut VirtQueue , num_packets : u16 , packet_size : u32) { for _ in 0 .. num_packets { let buff_tkn = match AvailBufferToken :: new (SmallVec :: new () , SmallVec :: from_buf ([BufferElem :: Sized (Box :: < Hdr , _ > :: new_uninit_in (DeviceAlloc)) , BufferElem :: Vector (Vec :: with_capacity_in (packet_size . try_into () . unwrap () , DeviceAlloc ,)) ,]) ,) { Ok (tkn) => tkn , Err (_vq_err) => { error ! ("Setup of network queue failed, which should not happen!") ; panic ! ("setup of network queue failed!") ; } } ; if let Err (err) = vq . dispatch (buff_tkn , false , BufferType :: Direct) { error ! ("{err:#?}") ; break ; } } }
};
}
