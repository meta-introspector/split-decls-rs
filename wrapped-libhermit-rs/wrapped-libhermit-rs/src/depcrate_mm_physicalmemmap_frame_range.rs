// Generated macro for map_frame_range (function)
macro_rules! Depcrate_mm_physicalmemmap_frame_range {
() => {
// Module: crate::mm::physicalmem
// Provides: {"map_frame_range"}
// Dependencies: {}
pub unsafe fn map_frame_range (frame_range : PageRange) { cfg_if :: cfg_if ! { if # [cfg (target_arch = "aarch64")] { type IdentityPageSize = crate :: arch :: mm :: paging :: BasePageSize ; } else if # [cfg (target_arch = "riscv64")] { type IdentityPageSize = crate :: arch :: mm :: paging :: HugePageSize ; } else if # [cfg (target_arch = "x86_64")] { type IdentityPageSize = crate :: arch :: mm :: paging :: LargePageSize ; } } let start = frame_range . start () . align_down (IdentityPageSize :: SIZE . try_into () . unwrap ()) ; let end = frame_range . end () . align_up (IdentityPageSize :: SIZE . try_into () . unwrap ()) ; (start .. end) . step_by (IdentityPageSize :: SIZE . try_into () . unwrap ()) . map (| addr | PhysAddr :: new (addr . try_into () . unwrap ())) . for_each (paging :: identity_map :: < IdentityPageSize >) ; if DeviceAlloc . phys_offset () != VirtAddr :: zero () { let flags = { let mut flags = PageTableEntryFlags :: empty () ; flags . normal () . writable () . execute_disable () ; flags } ; (start .. end) . step_by (IdentityPageSize :: SIZE . try_into () . unwrap ()) . for_each (| addr | { let phys_addr = PhysAddr :: new (addr . try_into () . unwrap ()) ; let virt_addr = VirtAddr :: from_ptr (DeviceAlloc . ptr_from :: < () > (phys_addr)) ; paging :: map :: < IdentityPageSize > (virt_addr , phys_addr , 1 , flags) ; }) ; } }
};
}
