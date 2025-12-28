macro_rules! Elf_Phdr {
    () => {
        # [cfg (target_pointer_width = "64")] # [repr (C)] pub struct Elf_Phdr { pub p_type : u32 , pub p_flags : u32 , pub p_offset : usize , pub p_vaddr : usize , pub p_paddr : usize , pub p_filesz : usize , pub p_memsz : usize , pub p_align : usize , }
    };
}

Elf_Phdr!();