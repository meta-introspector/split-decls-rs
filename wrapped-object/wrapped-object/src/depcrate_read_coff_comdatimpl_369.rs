// Generated macro for impl_369 (impl)
macro_rules! Depcrate_read_coff_comdatimpl_369 {
() => {
// Module: crate::read::coff::comdat
// Provides: {"impl_369"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > Iterator for CoffComdatSectionIterator < 'data , 'file , R , Coff > { type Item = SectionIndex ; fn next (& mut self) -> Option < Self :: Item > { loop { let index = self . index ; let symbol = self . file . common . symbols . symbol (index) . ok () ? ; self . index . 0 += 1 + symbol . number_of_aux_symbols () as usize ; if ! symbol . has_aux_section () { continue ; } let section_number = symbol . section_number () ; let aux = self . file . common . symbols . aux_section (index) . ok () ? ; if aux . selection == pe :: IMAGE_COMDAT_SELECT_ASSOCIATIVE { let number = if Coff :: is_type_bigobj () { u32 :: from (aux . number . get (LE)) | (u32 :: from (aux . high_number . get (LE)) << 16) } else { u32 :: from (aux . number . get (LE)) } ; if number as i32 == self . section_number { return Some (SectionIndex (section_number as usize)) ; } } else if aux . selection != 0 { if section_number == self . section_number { return Some (SectionIndex (section_number as usize)) ; } } } } }
};
}
