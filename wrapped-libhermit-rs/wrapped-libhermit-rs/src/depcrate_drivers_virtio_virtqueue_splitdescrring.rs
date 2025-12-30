// Generated macro for DescrRing (struct)
macro_rules! Depcrate_drivers_virtio_virtqueue_splitDescrRing {
() => {
// Module: crate::drivers::virtio::virtqueue::split
// Provides: {"DescrRing"}
// Dependencies: {}
struct DescrRing { read_idx : u16 , token_ring : Box < [Option < Box < TransferToken < virtq :: Desc > > >] > , mem_pool : MemPool , descr_table_cell : Box < UnsafeCell < [MaybeUninit < virtq :: Desc >] > , DeviceAlloc > , avail_ring_cell : Box < UnsafeCell < virtq :: Avail > , DeviceAlloc > , used_ring_cell : Box < UnsafeCell < virtq :: Used > , DeviceAlloc > , }
};
}
