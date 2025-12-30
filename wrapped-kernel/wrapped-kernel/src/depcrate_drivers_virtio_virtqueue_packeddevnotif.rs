// Generated macro for DevNotif (struct)
macro_rules! Depcrate_drivers_virtio_virtqueue_packedDevNotif {
() => {
// Module: crate::drivers::virtio::virtqueue::packed
// Provides: {"DevNotif"}
// Dependencies: {}
# [doc = " A type in order to implement the correct functionality upon"] # [doc = " the `EventSuppr` structure for device notifications settings."] # [doc = " The Device Event Suppression structure is read-only by the driver"] # [doc = " and controls the available buffer notifica- tions sent by the driver to the device."] struct DevNotif { # [doc = " Indicates if VIRTIO_F_RING_EVENT_IDX has been negotiated"] f_notif_idx : bool , # [doc = " Actual structure to read from, if device wants notifs"] raw : & 'static mut pvirtq :: EventSuppress , }
};
}
