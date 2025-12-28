macro_rules! Elf_Sym {
    () => {
        # [cfg (target_pointer_width = "64")] # [repr (C)] pub struct Elf_Sym { pub st_name : u32 , pub st_info : u8 , pub st_other : u8 , pub st_shndx : u16 , pub st_value : usize , pub st_size : usize , }
    };
}

Elf_Sym!()