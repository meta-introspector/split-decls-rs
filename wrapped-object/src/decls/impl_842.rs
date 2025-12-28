macro_rules! deps {
    () => {
        Rel!();
        Rel32!();
    };
}

macro_rules! impl_842 {
    () => {
        deps!();
        impl Rel for xcoff :: Rel32 { type Word = u32 ; fn r_vaddr (& self) -> Self :: Word { self . r_vaddr . get (BE) } fn r_symndx (& self) -> u32 { self . r_symndx . get (BE) } fn r_rsize (& self) -> u8 { self . r_rsize } fn r_rtype (& self) -> u8 { self . r_rtype } }
    };
}

impl_842!()