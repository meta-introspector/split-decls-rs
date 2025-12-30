// Generated macro for AttributesWriter (struct)
macro_rules! Depcrate_write_elf_writerAttributesWriter {
() => {
// Module: crate::write::elf::writer
// Provides: {"AttributesWriter"}
// Dependencies: {}
# [doc = " A helper for writing an attributes section."] # [doc = ""] # [doc = " Attributes have a variable length encoding, so it is awkward to write them in a"] # [doc = " single pass. Instead, we build the entire attributes section data in memory, using"] # [doc = " placeholders for unknown lengths that are filled in later."] # [allow (missing_debug_implementations)] pub struct AttributesWriter { endian : Endianness , data : Vec < u8 > , subsection_offset : usize , subsubsection_offset : usize , }
};
}
