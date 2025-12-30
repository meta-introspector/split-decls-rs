// Generated macro for impl_272 (impl)
macro_rules! Depcrate_machinst_bufferimpl_272 {
() => {
// Module: crate::machinst::buffer
// Provides: {"impl_272"}
// Dependencies: {}
impl < T : CompilePhase > MachBufferFinalized < T > { # [doc = " Get a list of source location mapping tuples in sorted-by-start-offset order."] pub fn get_srclocs_sorted (& self) -> & [T :: MachSrcLocType] { & self . srclocs [..] } # [doc = " Get the total required size for the code."] pub fn total_size (& self) -> CodeOffset { self . data . len () as CodeOffset } # [doc = " Return the code in this mach buffer as a hex string for testing purposes."] pub fn stringify_code_bytes (& self) -> String { use std :: fmt :: Write ; let mut s = String :: with_capacity (self . data . len () * 2) ; for b in & self . data { write ! (& mut s , "{b:02X}") . unwrap () ; } s } # [doc = " Get the code bytes."] pub fn data (& self) -> & [u8] { & self . data [..] } # [doc = " Get the list of external relocations for this code."] pub fn relocs (& self) -> & [FinalizedMachReloc] { & self . relocs [..] } # [doc = " Get the list of trap records for this code."] pub fn traps (& self) -> & [MachTrap] { & self . traps [..] } # [doc = " Get the user stack map metadata for this code."] pub fn user_stack_maps (& self) -> & [(CodeOffset , u32 , ir :: UserStackMap)] { & self . user_stack_maps } # [doc = " Take this buffer's user strack map metadata."] pub fn take_user_stack_maps (& mut self) -> SmallVec < [(CodeOffset , u32 , ir :: UserStackMap) ; 8] > { mem :: take (& mut self . user_stack_maps) } # [doc = " Get the list of call sites for this code."] pub fn call_sites (& self) -> & [MachCallSite] { & self . call_sites [..] } }
};
}
