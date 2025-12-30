// Generated macro for impl_203 (impl)
macro_rules! Depcrate_read_anyimpl_203 {
() => {
// Module: crate::read::any
// Provides: {"impl_203"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > > ObjectComdat < 'data > for Comdat < 'data , 'file , R > { type SectionIterator = ComdatSectionIterator < 'data , 'file , R > ; fn kind (& self) -> ComdatKind { with_inner ! (self . inner , ComdatInternal , | x | x . kind ()) } fn symbol (& self) -> SymbolIndex { with_inner ! (self . inner , ComdatInternal , | x | x . symbol ()) } fn name_bytes (& self) -> Result < & 'data [u8] > { with_inner ! (self . inner , ComdatInternal , | x | x . name_bytes ()) } fn name (& self) -> Result < & 'data str > { with_inner ! (self . inner , ComdatInternal , | x | x . name ()) } fn sections (& self) -> ComdatSectionIterator < 'data , 'file , R > { ComdatSectionIterator { inner : map_inner ! (self . inner , ComdatInternal , ComdatSectionIteratorInternal , | x | x . sections ()) , } } }
};
}
