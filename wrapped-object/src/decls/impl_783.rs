macro_rules! deps {
    () => {
        SectionHeader!();
        CsectAux!();
        Symbol64!();
        Rel64!();
        FileHeader!();
        FileHeader64!();
        FileAux64!();
        CsectAux64!();
        AuxHeader!();
        Symbol!();
        FileAux!();
        AuxHeader64!();
        SectionHeader64!();
        Rel!();
    };
}

macro_rules! impl_783 {
    () => {
        deps!();
        impl FileHeader for xcoff :: FileHeader64 { type Word = u64 ; type AuxHeader = xcoff :: AuxHeader64 ; type SectionHeader = xcoff :: SectionHeader64 ; type Symbol = xcoff :: Symbol64 ; type FileAux = xcoff :: FileAux64 ; type CsectAux = xcoff :: CsectAux64 ; type Rel = xcoff :: Rel64 ; fn is_type_64 (& self) -> bool { true } fn f_magic (& self) -> u16 { self . f_magic . get (BE) } fn f_nscns (& self) -> u16 { self . f_nscns . get (BE) } fn f_timdat (& self) -> u32 { self . f_timdat . get (BE) } fn f_symptr (& self) -> Self :: Word { self . f_symptr . get (BE) } fn f_nsyms (& self) -> u32 { self . f_nsyms . get (BE) } fn f_opthdr (& self) -> u16 { self . f_opthdr . get (BE) } fn f_flags (& self) -> u16 { self . f_flags . get (BE) } }
    };
}

impl_783!()