// Generated macro for Elf_Ehdr (struct)
macro_rules! Depcrate_elfElf_Ehdr {
() => {
// Module: crate::elf
// Provides: {"Elf_Ehdr"}
// Dependencies: {}
# [repr (C)] pub struct Elf_Ehdr { pub e_ident : [u8 ; EI_NIDENT] , pub e_type : u16 , pub e_machine : u16 , pub e_version : u32 , pub e_entry : usize , pub e_phoff : usize , pub e_shoff : usize , pub e_flags : u32 , pub e_ehsize : u16 , pub e_phentsize : u16 , pub e_phnum : u16 , pub e_shentsize : u16 , pub e_shnum : u16 , pub e_shstrndx : u16 , }
};
}
