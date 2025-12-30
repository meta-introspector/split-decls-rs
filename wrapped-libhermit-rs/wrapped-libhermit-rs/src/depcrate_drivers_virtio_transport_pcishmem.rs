// Generated macro for ShMem (struct)
macro_rules! Depcrate_drivers_virtio_transport_pciShMem {
() => {
// Module: crate::drivers::virtio::transport::pci
// Provides: {"ShMem"}
// Dependencies: {}
# [doc = " Defines a shared memory locate at location ptr with a length of len."] # [doc = " The shared memories Drop implementation does not dealloc the memory"] # [doc = " behind the pointer but sets it to zero, to prevent leakage of data."] struct ShMem { ptr : * mut u8 , len : usize , }
};
}
