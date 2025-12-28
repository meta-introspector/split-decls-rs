macro_rules! Elf_Verdaux {
    () => {
        # [repr (C)] pub struct Elf_Verdaux { pub vda_name : u32 , pub _vda_next : u32 , }
    };
}

Elf_Verdaux!();