macro_rules! Rel {
    () => {
        # [doc = " Unified native endian version of [`elf::Rel64`] and [`elf::Rela64`]."] # [allow (missing_docs)] # [derive (Debug , Clone)] pub struct Rel { pub r_offset : u64 , pub r_sym : u32 , pub r_type : u32 , pub r_addend : i64 , }
    };
}

Rel!();