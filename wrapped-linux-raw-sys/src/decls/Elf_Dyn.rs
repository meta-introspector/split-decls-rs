macro_rules! Elf_Dyn {
    () => {
        # [cfg (target_pointer_width = "64")] # [repr (C)] # [derive (Copy , Clone)] pub struct Elf_Dyn { pub d_tag : usize , pub d_un : Elf_Dyn_Union , }
    };
}

Elf_Dyn!()