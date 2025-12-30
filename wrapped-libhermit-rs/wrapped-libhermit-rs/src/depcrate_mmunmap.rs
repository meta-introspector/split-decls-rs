// Generated macro for unmap (function)
macro_rules! Depcrate_mmunmap {
() => {
// Module: crate::mm
// Provides: {"unmap"}
// Dependencies: {}
# [allow (dead_code)] # [doc = " unmaps virtual address, without 'freeing' physical memory it is mapped to!"] pub (crate) fn unmap (virtual_address : VirtAddr , size : usize) { let size = size . align_up (BasePageSize :: SIZE as usize) ; if arch :: mm :: paging :: virtual_to_physical (virtual_address) . is_some () { arch :: mm :: paging :: unmap :: < BasePageSize > (virtual_address , size / BasePageSize :: SIZE as usize ,) ; let range = PageRange :: from_start_len (virtual_address . as_usize () , size) . unwrap () ; unsafe { PageAlloc :: deallocate (range) ; } } else { panic ! ("No page table entry for virtual address {:p}" , virtual_address) ; } }
};
}
