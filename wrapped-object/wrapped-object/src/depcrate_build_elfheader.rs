// Generated macro for Header (struct)
macro_rules! Depcrate_build_elfHeader {
() => {
// Module: crate::build::elf
// Provides: {"Header"}
// Dependencies: {}
# [doc = " ELF file header."] # [doc = ""] # [doc = " This corresponds to fields in [`elf::FileHeader32`] or [`elf::FileHeader64`]."] # [doc = " This only contains the ELF file header fields that can be modified."] # [doc = " The other fields are automatically calculated."] # [derive (Debug , Default)] pub struct Header { # [doc = " The OS ABI field in the file header."] # [doc = ""] # [doc = " One of the `ELFOSABI*` constants."] pub os_abi : u8 , # [doc = " The ABI version field in the file header."] # [doc = ""] # [doc = " The meaning of this field depends on the `os_abi` value."] pub abi_version : u8 , # [doc = " The object file type in the file header."] # [doc = ""] # [doc = " One of the `ET_*` constants."] pub e_type : u16 , # [doc = " The architecture in the file header."] # [doc = ""] # [doc = " One of the `EM_*` constants."] pub e_machine : u16 , # [doc = " Entry point virtual address in the file header."] pub e_entry : u64 , # [doc = " The processor-specific flags in the file header."] # [doc = ""] # [doc = " A combination of the `EF_*` constants."] pub e_flags : u32 , # [doc = " The file offset of the program header table."] # [doc = ""] # [doc = " Writing will fail if the program header table cannot be placed at this offset."] pub e_phoff : u64 , }
};
}
