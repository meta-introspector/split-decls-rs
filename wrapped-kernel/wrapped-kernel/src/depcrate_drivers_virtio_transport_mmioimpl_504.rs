// Generated macro for impl_504 (impl)
macro_rules! Depcrate_drivers_virtio_transport_mmioimpl_504 {
() => {
// Module: crate::drivers::virtio::transport::mmio
// Provides: {"impl_504"}
// Dependencies: {}
impl NotifCfg { pub fn new (mut registers : VolatileRef < '_ , DeviceRegisters >) -> Self { let raw = registers . as_mut_ptr () . queue_notify () . as_raw_ptr () . as_ptr () ; NotifCfg { queue_notify : raw } } pub fn notification_location (& self , _vq_cfg_handler : & mut VqCfgHandler < '_ >) -> * mut le32 { self . queue_notify } }
};
}
