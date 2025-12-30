// Generated macro for Segment (struct)
macro_rules! Depcrate_build_elfSegment {
() => {
// Module: crate::build::elf
// Provides: {"Segment"}
// Dependencies: {}
# [doc = " A segment in [`Segments`]."] # [doc = ""] # [doc = " This corresponds to [`elf::ProgramHeader32`] or [`elf::ProgramHeader64`]."] # [derive (Debug)] pub struct Segment < 'data > { id : SegmentId , # [doc = " Ignore this segment when writing the ELF file."] pub delete : bool , # [doc = " The `p_type` field in the ELF program header."] # [doc = ""] # [doc = " One of the `PT_*` constants."] pub p_type : u32 , # [doc = " The `p_flags` field in the ELF program header."] # [doc = ""] # [doc = " A combination of the `PF_*` constants."] pub p_flags : u32 , # [doc = " The `p_offset` field in the ELF program header."] # [doc = ""] # [doc = " This is the file offset of the data in the segment. This should"] # [doc = " correspond to the file offset of the sections that are placed in"] # [doc = " this segment. Currently there is no support for section data"] # [doc = " that is not contained in sections."] pub p_offset : u64 , # [doc = " The `p_vaddr` field in the ELF program header."] pub p_vaddr : u64 , # [doc = " The `p_paddr` field in the ELF program header."] pub p_paddr : u64 , # [doc = " The `p_filesz` field in the ELF program header."] pub p_filesz : u64 , # [doc = " The `p_memsz` field in the ELF program header."] pub p_memsz : u64 , # [doc = " The `p_align` field in the ELF program header."] pub p_align : u64 , # [doc = " The sections contained in this segment."] pub sections : Vec < SectionId > , marker : PhantomData < & 'data () > , }
};
}
