macro_rules! deps {
    () => {
        ComdatKind!();
        SymbolIndex!();
        ReadRef!();
        ObjectComdat!();
        PeComdatSectionIterator!();
        PeComdat!();
        ImageNtHeaders!();
        SectionIterator!();
        Result!();
    };
}

macro_rules! impl_642 {
    () => {
        deps!();
        impl < 'data , 'file , Pe , R > ObjectComdat < 'data > for PeComdat < 'data , 'file , Pe , R > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { type SectionIterator = PeComdatSectionIterator < 'data , 'file , Pe , R > ; # [inline] fn kind (& self) -> ComdatKind { unreachable ! () ; } # [inline] fn symbol (& self) -> SymbolIndex { unreachable ! () ; } # [inline] fn name_bytes (& self) -> Result < & 'data [u8] > { unreachable ! () ; } # [inline] fn name (& self) -> Result < & 'data str > { unreachable ! () ; } # [inline] fn sections (& self) -> Self :: SectionIterator { unreachable ! () ; } }
    };
}

impl_642!()