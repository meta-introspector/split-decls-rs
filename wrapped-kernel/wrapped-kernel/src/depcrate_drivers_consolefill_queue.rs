// Generated macro for fill_queue (function)
macro_rules! Depcrate_drivers_consolefill_queue {
() => {
// Module: crate::drivers::console
// Provides: {"fill_queue"}
// Dependencies: {}
fn fill_queue (vq : & mut VirtQueue , num_packets : u16 , packet_size : u32) { for _ in 0 .. num_packets { let buff_tkn = match AvailBufferToken :: new (SmallVec :: new () , { let mut vec = SmallVec :: new () ; vec . push (BufferElem :: Vector (Vec :: with_capacity_in (packet_size . try_into () . unwrap () , DeviceAlloc ,))) ; vec }) { Ok (tkn) => tkn , Err (_vq_err) => { panic ! ("Setup of console queue failed, which should not happen!") ; } } ; if let Err (err) = vq . dispatch (buff_tkn , false , BufferType :: Direct) { error ! ("{err:#?}") ; break ; } } }
};
}
