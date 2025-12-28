macro_rules! deps {
    () => {
        MachOComdat!();
        SymbolIndex!();
        Result!();
        SectionIterator!();
        ComdatKind!();
        ObjectComdat!();
        ReadRef!();
        MachOComdatSectionIterator!();
        MachHeader!();
    };
}

macro_rules! impl_541 {
    () => {
        deps!();
        impl < 'data , 'file , Mach , R > ObjectComdat < 'data > for MachOComdat < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { type SectionIterator = MachOComdatSectionIterator < 'data , 'file , Mach , R > ; # [inline] fn kind (& self) -> ComdatKind { unreachable ! () ; } # [inline] fn symbol (& self) -> SymbolIndex { unreachable ! () ; } # [inline] fn name_bytes (& self) -> Result < & 'data [u8] > { unreachable ! () ; } # [inline] fn name (& self) -> Result < & 'data str > { unreachable ! () ; } # [inline] fn sections (& self) -> Self :: SectionIterator { unreachable ! () ; } }
    };
}

impl_541!()