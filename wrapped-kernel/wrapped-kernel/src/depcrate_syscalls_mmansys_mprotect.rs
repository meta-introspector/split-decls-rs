// Generated macro for sys_mprotect (function)
macro_rules! Depcrate_syscalls_mmansys_mprotect {
() => {
// Module: crate::syscalls::mman
// Provides: {"sys_mprotect"}
// Dependencies: {}
# [doc = " Configures the protections associated with a region of virtual memory"] # [doc = " starting at `ptr` and going to `size`."] # [doc = ""] # [doc = " Returns 0 on success and an error code on failure."] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_mprotect (ptr : * mut u8 , size : usize , prot_flags : MemoryProtection) -> i32 { let count = size / BasePageSize :: SIZE as usize ; let mut flags = PageTableEntryFlags :: empty () ; flags . normal () . writable () ; if prot_flags . contains (MemoryProtection :: Write) { flags . writable () ; } if ! prot_flags . contains (MemoryProtection :: Exec) { flags . execute_disable () ; } let virtual_address = VirtAddr :: from_ptr (ptr) ; debug ! ("Mprotect {virtual_address:X} ({size}) -> {prot_flags:?})") ; if let Some (physical_address) = arch :: mm :: paging :: virtual_to_physical (virtual_address) { arch :: mm :: paging :: map :: < BasePageSize > (virtual_address , physical_address , count , flags) ; 0 } else { let frame_layout = PageLayout :: from_size (size) . unwrap () ; let frame_range = FrameAlloc :: allocate (frame_layout) . unwrap () ; let physical_address = PhysAddr :: from (frame_range . start ()) ; arch :: mm :: paging :: map :: < BasePageSize > (virtual_address , physical_address , count , flags) ; 0 } }
};
}
