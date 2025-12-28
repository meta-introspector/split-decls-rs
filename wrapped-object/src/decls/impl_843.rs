macro_rules! deps {
    () => {
        Rel!();
        Rel64!();
    };
}

macro_rules! impl_843 {
    () => {
        deps!();
        impl Rel for xcoff :: Rel64 { type Word = u64 ; fn r_vaddr (& self) -> Self :: Word { self . r_vaddr . get (BE) } fn r_symndx (& self) -> u32 { self . r_symndx . get (BE) } fn r_rsize (& self) -> u8 { self . r_rsize } fn r_rtype (& self) -> u8 { self . r_rtype } }
    };
}

impl_843!()