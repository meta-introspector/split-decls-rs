// Generated macro for DrvNotif (struct)
macro_rules! Depcrate_drivers_virtio_virtqueue_packedDrvNotif {
() => {
// Module: crate::drivers::virtio::virtqueue::packed
// Provides: {"DrvNotif"}
// Dependencies: {}
# [doc = " A type in order to implement the correct functionality upon"] # [doc = " the `EventSuppr` structure for driver notifications settings."] # [doc = " The Driver Event Suppression structure is read-only by the device"] # [doc = " and controls the used buffer notifications sent by the device to the driver."] struct DrvNotif { # [doc = " Indicates if VIRTIO_F_RING_EVENT_IDX has been negotiated"] f_notif_idx : bool , # [doc = " Actual structure to read from, if device wants notifs"] raw : & 'static mut pvirtq :: EventSuppress , }
};
}
