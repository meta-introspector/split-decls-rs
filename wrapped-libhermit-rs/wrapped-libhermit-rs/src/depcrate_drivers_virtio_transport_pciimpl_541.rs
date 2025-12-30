// Generated macro for impl_541 (impl)
macro_rules! Depcrate_drivers_virtio_transport_pciimpl_541 {
() => {
// Module: crate::drivers::virtio::transport::pci
// Provides: {"impl_541"}
// Dependencies: {}
impl VqCfgHandler < '_ > { fn select_queue (& mut self) { self . raw . as_mut_ptr () . queue_select () . write (self . vq_index . into ()) ; } # [doc = " Sets the size of a given virtqueue. In case the provided size exceeds the maximum allowed"] # [doc = " size, the size is set to this maximum instead. Else size is set to the provided value."] # [doc = ""] # [doc = " Returns the set size in form of a `u16`."] pub fn set_vq_size (& mut self , size : u16) -> u16 { self . select_queue () ; let queue_size = self . raw . as_mut_ptr () . queue_size () ; if queue_size . read () . to_ne () >= size { queue_size . write (size . into ()) ; } queue_size . read () . to_ne () } pub fn set_ring_addr (& mut self , addr : PhysAddr) { self . select_queue () ; self . raw . as_mut_ptr () . queue_desc () . write (addr . as_u64 () . into ()) ; } pub fn set_drv_ctrl_addr (& mut self , addr : PhysAddr) { self . select_queue () ; self . raw . as_mut_ptr () . queue_driver () . write (addr . as_u64 () . into ()) ; } pub fn set_dev_ctrl_addr (& mut self , addr : PhysAddr) { self . select_queue () ; self . raw . as_mut_ptr () . queue_device () . write (addr . as_u64 () . into ()) ; } pub fn notif_off (& mut self) -> u16 { self . select_queue () ; self . raw . as_mut_ptr () . queue_notify_off () . read () . to_ne () } pub fn enable_queue (& mut self) { self . select_queue () ; self . raw . as_mut_ptr () . queue_enable () . write (1 . into ()) ; } }
};
}
