macro_rules! deps {
    () => {
        WasmComdatSectionIterator!();
        WasmComdat!();
        SectionIterator!();
        Result!();
        ObjectComdat!();
        SymbolIndex!();
        ComdatKind!();
    };
}

macro_rules! impl_760 {
    () => {
        deps!();
        impl < 'data , 'file , R > ObjectComdat < 'data > for WasmComdat < 'data , 'file , R > { type SectionIterator = WasmComdatSectionIterator < 'data , 'file , R > ; # [inline] fn kind (& self) -> ComdatKind { unreachable ! () ; } # [inline] fn symbol (& self) -> SymbolIndex { unreachable ! () ; } # [inline] fn name_bytes (& self) -> Result < & 'data [u8] > { unreachable ! () ; } # [inline] fn name (& self) -> Result < & 'data str > { unreachable ! () ; } # [inline] fn sections (& self) -> Self :: SectionIterator { unreachable ! () ; } }
    };
}

impl_760!()