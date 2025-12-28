macro_rules! deps {
    () => {
        Elf_Rel!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl Elf_Rel { # [inline] pub fn type_ (& self) -> u32 { # [cfg (target_pointer_width = "32")] { self . r_info & 0xff } # [cfg (target_pointer_width = "64")] { (self . r_info & 0xffff_ffff) as u32 } } }
    };
}

impl_87!()