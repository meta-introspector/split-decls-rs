// Generated macro for impl_770 (impl)
macro_rules! Depcrate_read_macho_fileimpl_770 {
() => {
// Module: crate::read::macho::file
// Provides: {"impl_770"}
// Dependencies: {}
impl < 'data , 'file , Mach , R > ObjectComdat < 'data > for MachOComdat < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { type SectionIterator = MachOComdatSectionIterator < 'data , 'file , Mach , R > ; # [inline] fn kind (& self) -> ComdatKind { unreachable ! () ; } # [inline] fn symbol (& self) -> SymbolIndex { unreachable ! () ; } # [inline] fn name_bytes (& self) -> Result < & 'data [u8] > { unreachable ! () ; } # [inline] fn name (& self) -> Result < & 'data str > { unreachable ! () ; } # [inline] fn sections (& self) -> Self :: SectionIterator { unreachable ! () ; } }
};
}
