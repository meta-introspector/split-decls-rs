macro_rules! Elf_Ehdr {
    () => {
        # [repr (C)] pub struct Elf_Ehdr { pub e_ident : [u8 ; EI_NIDENT] , pub e_type : u16 , pub e_machine : u16 , pub e_version : u32 , pub e_entry : usize , pub e_phoff : usize , pub e_shoff : usize , pub e_flags : u32 , pub e_ehsize : u16 , pub e_phentsize : u16 , pub e_phnum : u16 , pub e_shentsize : u16 , pub e_shnum : u16 , pub e_shstrndx : u16 , }
    };
}

Elf_Ehdr!()