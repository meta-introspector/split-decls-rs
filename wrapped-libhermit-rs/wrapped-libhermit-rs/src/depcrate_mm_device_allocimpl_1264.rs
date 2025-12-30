// Generated macro for impl_1264 (impl)
macro_rules! Depcrate_mm_device_allocimpl_1264 {
() => {
// Module: crate::mm::device_alloc
// Provides: {"impl_1264"}
// Dependencies: {}
unsafe impl Allocator for DeviceAlloc { fn allocate (& self , layout : Layout) -> Result < NonNull < [u8] > , AllocError > { assert ! (layout . align () <= BasePageSize :: SIZE as usize) ; let size = layout . size () . align_up (BasePageSize :: SIZE as usize) ; let frame_layout = PageLayout :: from_size (size) . unwrap () ; let frame_range = FrameAlloc :: allocate (frame_layout) . map_err (| _ | AllocError) ? ; let phys_addr = PhysAddr :: from (frame_range . start ()) ; let ptr = self . ptr_from (phys_addr) ; let slice = ptr :: slice_from_raw_parts_mut (ptr , size) ; Ok (NonNull :: new (slice) . unwrap ()) } unsafe fn deallocate (& self , ptr : NonNull < u8 > , layout : Layout) { assert ! (layout . align () <= BasePageSize :: SIZE as usize) ; let size = layout . size () . align_up (BasePageSize :: SIZE as usize) ; let phys_addr = self . phys_addr_from (ptr . as_ptr ()) ; let range = PageRange :: from_start_len (phys_addr . as_usize () , size) . unwrap () ; unsafe { FrameAlloc :: deallocate (range) ; } } }
};
}
