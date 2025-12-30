// Generated macro for sys_mmap (function)
macro_rules! Depcrate_syscalls_mmansys_mmap {
() => {
// Module: crate::syscalls::mman
// Provides: {"sys_mmap"}
// Dependencies: {}
# [doc = " Creates a new virtual memory mapping of the `size` specified with"] # [doc = " protection bits specified in `prot_flags`."] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_mmap (size : usize , prot_flags : MemoryProtection , ret : & mut * mut u8) -> i32 { let size = size . align_up (BasePageSize :: SIZE as usize) ; let layout = PageLayout :: from_size (size) . unwrap () ; let page_range = PageAlloc :: allocate (layout) . unwrap () ; let virtual_address = VirtAddr :: from (page_range . start ()) ; if prot_flags . is_empty () { * ret = virtual_address . as_mut_ptr () ; unsafe { PROT_NONE_FREE_LIST . lock () . deallocate (page_range) . unwrap () ; } return 0 ; } let frame_layout = PageLayout :: from_size (size) . unwrap () ; let frame_range = FrameAlloc :: allocate (frame_layout) . unwrap () ; let physical_address = PhysAddr :: from (frame_range . start ()) ; debug ! ("Mmap {physical_address:X} -> {virtual_address:X} ({size})") ; let count = size / BasePageSize :: SIZE as usize ; let mut flags = PageTableEntryFlags :: empty () ; flags . normal () . writable () ; if prot_flags . contains (MemoryProtection :: Write) { flags . writable () ; } if ! prot_flags . contains (MemoryProtection :: Exec) { flags . execute_disable () ; } arch :: mm :: paging :: map :: < BasePageSize > (virtual_address , physical_address , count , flags) ; * ret = virtual_address . as_mut_ptr () ; 0 }
};
}
