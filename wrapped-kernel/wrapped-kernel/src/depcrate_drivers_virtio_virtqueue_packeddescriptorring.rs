// Generated macro for DescriptorRing (struct)
macro_rules! Depcrate_drivers_virtio_virtqueue_packedDescriptorRing {
() => {
// Module: crate::drivers::virtio::virtqueue::packed
// Provides: {"DescriptorRing"}
// Dependencies: {}
# [doc = " Structure which allows to control raw ring and operate easily on it"] struct DescriptorRing { ring : Box < [pvirtq :: Desc] , DeviceAlloc > , tkn_ref_ring : Box < [Option < TransferToken < pvirtq :: Desc > >] > , # [doc = " where to insert available descriptors next"] write_index : u16 , # [doc = " How much descriptors can be inserted"] capacity : u16 , # [doc = " Where to expect the next used descriptor by the device"] poll_index : u16 , # [doc = " See Virtio specification v1.1. - 2.7.1"] drv_wc : bool , dev_wc : bool , # [doc = " Memory pool controls the amount of \"free floating\" descriptors"] # [doc = " See [MemPool] docs for detail."] mem_pool : MemPool , }
};
}
