// Generated macro for map (function)
macro_rules! Depcrate_mmmap {
() => {
// Module: crate::mm
// Provides: {"map"}
// Dependencies: {}
# [doc = " Maps a given physical address and size in virtual space and returns address."] # [cfg (feature = "pci")] pub (crate) fn map (physical_address : PhysAddr , size : usize , writable : bool , no_execution : bool , no_cache : bool ,) -> VirtAddr { use crate :: arch :: mm :: paging :: PageTableEntryFlags ; # [cfg (target_arch = "x86_64")] use crate :: arch :: mm :: paging :: PageTableEntryFlagsExt ; let size = size . align_up (BasePageSize :: SIZE as usize) ; let count = size / BasePageSize :: SIZE as usize ; let mut flags = PageTableEntryFlags :: empty () ; flags . normal () ; if writable { flags . writable () ; } if no_execution { flags . execute_disable () ; } if no_cache { flags . device () ; } let layout = PageLayout :: from_size (size) . unwrap () ; let page_range = PageAlloc :: allocate (layout) . unwrap () ; let virtual_address = VirtAddr :: from (page_range . start ()) ; arch :: mm :: paging :: map :: < BasePageSize > (virtual_address , physical_address , count , flags) ; virtual_address }
};
}
