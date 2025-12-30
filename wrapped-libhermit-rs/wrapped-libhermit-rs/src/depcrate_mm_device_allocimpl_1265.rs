// Generated macro for impl_1265 (impl)
macro_rules! Depcrate_mm_device_allocimpl_1265 {
() => {
// Module: crate::mm::device_alloc
// Provides: {"impl_1265"}
// Dependencies: {}
impl DeviceAlloc { # [doc = " Returns a pointer corresponding to `phys_addr`."] # [inline] pub fn ptr_from < T > (& self , phys_addr : PhysAddr) -> * mut T { let addr = phys_addr . as_usize () + self . phys_offset () . as_usize () ; ptr :: with_exposed_provenance_mut (addr) } # [doc = " Returns the physical address of `ptr`."] # [doc = ""] # [doc = " The address is only correct if `ptr` has been allocated by this allocator."] # [inline] pub fn phys_addr_from < T : ? Sized > (& self , ptr : * mut T) -> PhysAddr { let addr = u64 :: try_from (ptr . expose_provenance ()) . unwrap () - self . phys_offset () . as_u64 () ; PhysAddr :: new (addr) } # [doc = " Returns the physical address offset."] # [doc = ""] # [doc = " This device allocator expects the complete physical memory to be mapped device-readable at this offset."] # [inline] pub fn phys_offset (& self) -> VirtAddr { if cfg ! (careful) { virtualmem :: kernel_heap_end () . as_u64 () . div_ceil (4) . into () } else { 0u64 . into () } } }
};
}
