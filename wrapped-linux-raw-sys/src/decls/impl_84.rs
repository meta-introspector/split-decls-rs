macro_rules! deps {
    () => {
        Elf_Rela!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl Elf_Rela { # [inline] pub fn type_ (& self) -> u32 { # [cfg (target_pointer_width = "32")] { self . r_info & 0xff } # [cfg (target_pointer_width = "64")] { (self . r_info & 0xffff_ffff) as u32 } } }
    };
}

impl_84!();