// Generated macro for MemPool (struct)
macro_rules! Depcrate_drivers_virtio_virtqueueMemPool {
() => {
// Module: crate::drivers::virtio::virtqueue
// Provides: {"MemPool"}
// Dependencies: {}
# [doc = " MemPool allows to easily control, request and provide memory for Virtqueues."] # [doc = ""] # [doc = " The struct is initialized with a limit of free running \"tracked\""] # [doc = " memory descriptor ids. As Virtqueus do only allow a limited amount of descriptors in their queue,"] # [doc = " the independent queues, can control the number of descriptors by this."] struct MemPool { pool : Vec < u16 > , limit : u16 , }
};
}
