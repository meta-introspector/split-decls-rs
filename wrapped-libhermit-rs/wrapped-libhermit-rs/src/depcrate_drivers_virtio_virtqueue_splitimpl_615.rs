// Generated macro for impl_615 (impl)
macro_rules! Depcrate_drivers_virtio_virtqueue_splitimpl_615 {
() => {
// Module: crate::drivers::virtio::virtqueue::split
// Provides: {"impl_615"}
// Dependencies: {}
impl Virtq for SplitVq { fn enable_notifs (& mut self) { self . ring . drv_enable_notif () ; } fn disable_notifs (& mut self) { self . ring . drv_disable_notif () ; } fn try_recv (& mut self) -> Result < UsedBufferToken , VirtqError > { self . ring . try_recv () } fn dispatch_batch (& mut self , _tkns : Vec < (AvailBufferToken , BufferType) > , _notif : bool ,) -> Result < () , VirtqError > { unimplemented ! () ; } fn dispatch_batch_await (& mut self , _tkns : Vec < (AvailBufferToken , BufferType) > , _notif : bool ,) -> Result < () , VirtqError > { unimplemented ! () } fn dispatch (& mut self , buffer_tkn : AvailBufferToken , notif : bool , buffer_type : BufferType ,) -> Result < () , VirtqError > { let transfer_tkn = Self :: transfer_token_from_buffer_token (buffer_tkn , buffer_type) ; let next_idx = self . ring . push (transfer_tkn) ? ; if notif { unimplemented ! () ; } if self . ring . dev_is_notif () { let notification_data = NotificationData :: new () . with_vqn (self . index) . with_next_idx (next_idx) ; self . notif_ctrl . notify_dev (notification_data) ; } Ok (()) } fn index (& self) -> u16 { self . index } fn size (& self) -> u16 { self . size } fn has_used_buffers (& self) -> bool { self . ring . read_idx != self . ring . used_ring () . idx . to_ne () } }
};
}
