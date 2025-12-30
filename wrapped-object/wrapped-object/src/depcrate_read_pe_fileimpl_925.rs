// Generated macro for impl_925 (impl)
macro_rules! Depcrate_read_pe_fileimpl_925 {
() => {
// Module: crate::read::pe::file
// Provides: {"impl_925"}
// Dependencies: {}
impl < 'data , 'file , Pe , R > ObjectComdat < 'data > for PeComdat < 'data , 'file , Pe , R > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { type SectionIterator = PeComdatSectionIterator < 'data , 'file , Pe , R > ; # [inline] fn kind (& self) -> ComdatKind { unreachable ! () ; } # [inline] fn symbol (& self) -> SymbolIndex { unreachable ! () ; } # [inline] fn name_bytes (& self) -> Result < & 'data [u8] > { unreachable ! () ; } # [inline] fn name (& self) -> Result < & 'data str > { unreachable ! () ; } # [inline] fn sections (& self) -> Self :: SectionIterator { unreachable ! () ; } }
};
}
