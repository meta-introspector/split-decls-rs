// Generated macro for get_aux_max_alignment (function)
macro_rules! Depcrate_object_readerget_aux_max_alignment {
() => {
// Module: crate::object_reader
// Provides: {"get_aux_max_alignment"}
// Dependencies: {}
fn get_aux_max_alignment < AuxiliaryHeader : object :: read :: xcoff :: AuxHeader > (aux_header_size : u16 , aux_header : Option < & AuxiliaryHeader > , log_2_of_max_align : u32 , offset_of_modtype : u16 ,) -> u32 { let Some (aux_header) = aux_header else { return MIN_BIG_ARCHIVE_MEM_DATA_ALIGN ; } ; if aux_header_size < offset_of_modtype { return MIN_BIG_ARCHIVE_MEM_DATA_ALIGN ; } if aux_header . o_snloader () == 0 { return MIN_BIG_ARCHIVE_MEM_DATA_ALIGN ; } let log_2_of_align = u32 :: from (std :: cmp :: max (aux_header . o_algntext () , aux_header . o_algndata () ,)) ; 1 << (if log_2_of_align > LOG2_OF_AIXPAGE_SIZE { log_2_of_max_align } else { log_2_of_align }) }
};
}
