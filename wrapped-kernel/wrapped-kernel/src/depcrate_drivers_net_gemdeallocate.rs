// Generated macro for deallocate (function)
macro_rules! Depcrate_drivers_net_gemdeallocate {
() => {
// Module: crate::drivers::net::gem
// Provides: {"deallocate"}
// Dependencies: {}
# [doc = " Soft-deprecated in favor of `DeviceAlloc`"] unsafe fn deallocate (virtual_address : VirtAddr , size : usize) { let layout = Layout :: from_size_align (size , 8) . unwrap () ; let ptr = NonNull :: new (virtual_address . as_mut_ptr :: < u8 > ()) . unwrap () ; unsafe { DeviceAlloc . deallocate (ptr , layout) ; } }
};
}
