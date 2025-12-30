// Generated macro for impl_364 (impl)
macro_rules! Depcrate_read_coff_comdatimpl_364 {
() => {
// Module: crate::read::coff::comdat
// Provides: {"impl_364"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > CoffComdat < 'data , 'file , R , Coff > { fn parse (file : & 'file CoffFile < 'data , R , Coff > , section_symbol : & 'data Coff :: ImageSymbol , index : SymbolIndex ,) -> Option < CoffComdat < 'data , 'file , R , Coff > > { if ! section_symbol . has_aux_section () { return None ; } let aux = file . common . symbols . aux_section (index) . ok () ? ; let selection = aux . selection ; if selection == 0 || selection == pe :: IMAGE_COMDAT_SELECT_ASSOCIATIVE { return None ; } let mut symbol_index = index ; let mut symbol = section_symbol ; let section_number = section_symbol . section_number () ; loop { symbol_index . 0 += 1 + symbol . number_of_aux_symbols () as usize ; symbol = file . common . symbols . symbol (symbol_index) . ok () ? ; if section_number == symbol . section_number () { break ; } } Some (CoffComdat { file , symbol_index , symbol , selection , }) } }
};
}
