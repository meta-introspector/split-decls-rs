macro_rules! deps {
    () => {
        FileHeader32!();
        FileAux32!();
        FileHeader!();
        SectionHeader32!();
        FileAux!();
        AuxHeader!();
        Symbol32!();
        CsectAux32!();
        Rel!();
        CsectAux!();
        AuxHeader32!();
        Symbol!();
        SectionHeader!();
        Rel32!();
    };
}

macro_rules! impl_782 {
    () => {
        deps!();
        impl FileHeader for xcoff :: FileHeader32 { type Word = u32 ; type AuxHeader = xcoff :: AuxHeader32 ; type SectionHeader = xcoff :: SectionHeader32 ; type Symbol = xcoff :: Symbol32 ; type FileAux = xcoff :: FileAux32 ; type CsectAux = xcoff :: CsectAux32 ; type Rel = xcoff :: Rel32 ; fn is_type_64 (& self) -> bool { false } fn f_magic (& self) -> u16 { self . f_magic . get (BE) } fn f_nscns (& self) -> u16 { self . f_nscns . get (BE) } fn f_timdat (& self) -> u32 { self . f_timdat . get (BE) } fn f_symptr (& self) -> Self :: Word { self . f_symptr . get (BE) } fn f_nsyms (& self) -> u32 { self . f_nsyms . get (BE) } fn f_opthdr (& self) -> u16 { self . f_opthdr . get (BE) } fn f_flags (& self) -> u16 { self . f_flags . get (BE) } }
    };
}

impl_782!();