// Generated macro for print_elf (function)
macro_rules! Depcrate_readobj_elfprint_elf {
() => {
// Module: crate::readobj::elf
// Provides: {"print_elf"}
// Dependencies: {}
fn print_elf < Elf : FileHeader < Endian = Endianness > > (p : & mut Printer < '_ > , elf : & Elf , data : & [u8]) { if let Some (endian) = elf . endian () . print_err (p) { print_file_header (p , endian , elf) ; if let Some (segments) = elf . program_headers (endian , data) . print_err (p) { print_program_headers (p , endian , data , elf , segments) ; } if let Some (sections) = elf . sections (endian , data) . print_err (p) { print_section_headers (p , endian , data , elf , & sections) ; } } }
};
}
