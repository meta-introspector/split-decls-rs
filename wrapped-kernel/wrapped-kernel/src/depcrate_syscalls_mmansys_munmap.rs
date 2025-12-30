// Generated macro for sys_munmap (function)
macro_rules! Depcrate_syscalls_mmansys_munmap {
() => {
// Module: crate::syscalls::mman
// Provides: {"sys_munmap"}
// Dependencies: {}
# [doc = " Unmaps memory at the specified `ptr` for `size` bytes."] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_munmap (ptr : * mut u8 , size : usize) -> i32 { let virtual_address = VirtAddr :: from_ptr (ptr) ; let size = size . align_up (BasePageSize :: SIZE as usize) ; let page_range = PageRange :: from_start_len (virtual_address . as_usize () , size) . unwrap () ; if PROT_NONE_FREE_LIST . lock () . allocate_at (page_range) . is_ok () { return 0 ; } if let Some (physical_address) = arch :: mm :: paging :: virtual_to_physical (virtual_address) { arch :: mm :: paging :: unmap :: < BasePageSize > (virtual_address , size / BasePageSize :: SIZE as usize ,) ; debug ! ("Unmapping {virtual_address:X} ({size}) -> {physical_address:X}") ; let frame_range = PageRange :: from_start_len (physical_address . as_u64 () as usize , size) . unwrap () ; unsafe { FrameAlloc :: deallocate (frame_range) ; } } unsafe { PageAlloc :: deallocate (page_range) ; } 0 }
};
}
