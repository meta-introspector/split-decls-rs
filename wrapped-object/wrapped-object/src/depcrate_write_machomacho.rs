// Generated macro for MachO (trait)
macro_rules! Depcrate_write_machoMachO {
() => {
// Module: crate::write::macho
// Provides: {"MachO"}
// Dependencies: {}
trait MachO { fn mach_header_size (& self) -> usize ; fn segment_command_size (& self) -> usize ; fn section_header_size (& self) -> usize ; fn nlist_size (& self) -> usize ; fn write_mach_header (& self , buffer : & mut dyn WritableBuffer , section : MachHeader) ; fn write_segment_command (& self , buffer : & mut dyn WritableBuffer , segment : SegmentCommand) ; fn write_section (& self , buffer : & mut dyn WritableBuffer , section : SectionHeader) ; fn write_nlist (& self , buffer : & mut dyn WritableBuffer , nlist : Nlist) ; }
};
}
