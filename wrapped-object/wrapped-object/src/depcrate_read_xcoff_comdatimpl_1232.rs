// Generated macro for impl_1232 (impl)
macro_rules! Depcrate_read_xcoff_comdatimpl_1232 {
() => {
// Module: crate::read::xcoff::comdat
// Provides: {"impl_1232"}
// Dependencies: {}
impl < 'data , 'file , Xcoff , R > ObjectComdat < 'data > for XcoffComdat < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { type SectionIterator = XcoffComdatSectionIterator < 'data , 'file , Xcoff , R > ; # [inline] fn kind (& self) -> ComdatKind { unreachable ! () ; } # [inline] fn symbol (& self) -> SymbolIndex { unreachable ! () ; } # [inline] fn name_bytes (& self) -> Result < & 'data [u8] > { unreachable ! () ; } # [inline] fn name (& self) -> Result < & 'data str > { unreachable ! () ; } # [inline] fn sections (& self) -> Self :: SectionIterator { unreachable ! () ; } }
};
}
