// Generated macro for IsrStatus (struct)
macro_rules! Depcrate_drivers_virtio_transport_mmioIsrStatus {
() => {
// Module: crate::drivers::virtio::transport::mmio
// Provides: {"IsrStatus"}
// Dependencies: {}
# [doc = " Wraps a [`DeviceRegisters`] in order to preserve"] # [doc = " the original structure and allow interaction with the device via"] # [doc = " the structure."] # [doc = ""] # [doc = " Provides a safe API for Raw structure and allows interaction with the device via"] # [doc = " the structure."] pub struct IsrStatus { raw : VolatileRef < 'static , DeviceRegisters > , }
};
}
