// Generated macro for impl_649 (impl)
macro_rules! Depcrate_drivers_virtio_virtqueueimpl_649 {
() => {
// Module: crate::drivers::virtio::virtqueue
// Provides: {"impl_649"}
// Dependencies: {}
impl MemPool { # [doc = " Returns a given id to the id pool"] fn ret_id (& mut self , id : u16) { self . pool . push (id) ; } # [doc = " Returns a new instance, with a pool of the specified size."] fn new (size : u16) -> MemPool { MemPool { pool : (0 .. size) . collect () , limit : size , } } }
};
}
