// Generated macro for impl_499 (impl)
macro_rules! Depcrate_drivers_virtio_transport_mmioimpl_499 {
() => {
// Module: crate::drivers::virtio::transport::mmio
// Provides: {"impl_499"}
// Dependencies: {}
impl VqCfgHandler < '_ > { fn select_queue (& mut self) { self . raw . as_mut_ptr () . queue_sel () . write (self . vq_index . into ()) ; } # [doc = " Sets the size of a given virtqueue. In case the provided size exceeds the maximum allowed"] # [doc = " size, the size is set to this maximum instead. Else size is set to the provided value."] # [doc = ""] # [doc = " Returns the set size in form of a `u16`."] pub fn set_vq_size (& mut self , size : u16) -> u16 { self . select_queue () ; let ptr = self . raw . as_mut_ptr () ; let num_max = ptr . queue_num_max () . read () . to_ne () ; let size = size . min (num_max) ; ptr . queue_num () . write (size . into ()) ; size } pub fn set_ring_addr (& mut self , addr : PhysAddr) { self . select_queue () ; self . raw . as_mut_ptr () . queue_desc () . write (addr . as_u64 () . into ()) ; } pub fn set_drv_ctrl_addr (& mut self , addr : PhysAddr) { self . select_queue () ; self . raw . as_mut_ptr () . queue_driver () . write (addr . as_u64 () . into ()) ; } pub fn set_dev_ctrl_addr (& mut self , addr : PhysAddr) { self . select_queue () ; self . raw . as_mut_ptr () . queue_device () . write (addr . as_u64 () . into ()) ; } pub fn enable_queue (& mut self) { self . select_queue () ; self . raw . as_mut_ptr () . queue_ready () . write (true) ; } }
};
}
