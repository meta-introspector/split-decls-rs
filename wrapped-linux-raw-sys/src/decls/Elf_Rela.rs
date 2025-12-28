macro_rules! Elf_Rela {
    () => {
        # [cfg (target_pointer_width = "64")] # [repr (C)] pub struct Elf_Rela { pub r_offset : usize , pub r_info : u64 , pub r_addend : usize , }
    };
}

Elf_Rela!();