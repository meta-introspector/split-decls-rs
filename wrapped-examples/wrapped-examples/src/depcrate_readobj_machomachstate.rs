// Generated macro for MachState (struct)
macro_rules! Depcrate_readobj_machoMachState {
() => {
// Module: crate::readobj::macho
// Provides: {"MachState"}
// Dependencies: {}
# [derive (Default)] struct MachState < 'a > { cputype : u32 , linkedit_data : & 'a [u8] , symbols : Vec < Option < & 'a [u8] > > , sections : Vec < Vec < u8 > > , section_index : usize , text_segment_addr : u64 , }
};
}
