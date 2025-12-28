macro_rules! Elf_Verdef {
    () => {
        # [repr (C)] pub struct Elf_Verdef { pub vd_version : u16 , pub vd_flags : u16 , pub vd_ndx : u16 , pub vd_cnt : u16 , pub vd_hash : u32 , pub vd_aux : u32 , pub vd_next : u32 , }
    };
}

Elf_Verdef!();