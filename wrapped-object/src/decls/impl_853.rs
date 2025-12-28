macro_rules! deps {
    () => {
        FileHeader!();
        ReadRef!();
        SectionIterator!();
        SymbolIndex!();
        Result!();
        ObjectComdat!();
        ComdatKind!();
        XcoffComdatSectionIterator!();
        XcoffComdat!();
    };
}

macro_rules! impl_853 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff , R > ObjectComdat < 'data > for XcoffComdat < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { type SectionIterator = XcoffComdatSectionIterator < 'data , 'file , Xcoff , R > ; # [inline] fn kind (& self) -> ComdatKind { unreachable ! () ; } # [inline] fn symbol (& self) -> SymbolIndex { unreachable ! () ; } # [inline] fn name_bytes (& self) -> Result < & 'data [u8] > { unreachable ! () ; } # [inline] fn name (& self) -> Result < & 'data str > { unreachable ! () ; } # [inline] fn sections (& self) -> Self :: SectionIterator { unreachable ! () ; } }
    };
}

impl_853!()