// Generated macro for get_member_alignment (function)
macro_rules! Depcrate_object_readerget_member_alignment {
() => {
// Module: crate::object_reader
// Provides: {"get_member_alignment"}
// Dependencies: {}
pub fn get_member_alignment (obj : & [u8]) -> u32 { use object :: read :: xcoff :: FileHeader ; match object :: FileKind :: parse (obj) { Ok (object :: FileKind :: Xcoff64) => { let mut offset = 0 ; let Ok (header) = xcoff :: FileHeader64 :: parse (obj , & mut offset) else { return MIN_BIG_ARCHIVE_MEM_DATA_ALIGN ; } ; let Ok (aux_header) = header . aux_header (obj , & mut offset) else { return MIN_BIG_ARCHIVE_MEM_DATA_ALIGN ; } ; get_aux_max_alignment (header . f_opthdr () , aux_header , LOG2_OF_AIXPAGE_SIZE , offset_of ! (object :: xcoff :: AuxHeader64 , o_modtype) . try_into () . unwrap () ,) } Ok (object :: FileKind :: Xcoff32) => { let mut offset = 0 ; let Ok (header) = object :: xcoff :: FileHeader32 :: parse (obj , & mut offset) else { return MIN_BIG_ARCHIVE_MEM_DATA_ALIGN ; } ; let Ok (aux_header) = header . aux_header (obj , & mut offset) else { return MIN_BIG_ARCHIVE_MEM_DATA_ALIGN ; } ; get_aux_max_alignment (header . f_opthdr () , aux_header , 2 , offset_of ! (object :: xcoff :: AuxHeader32 , o_modtype) . try_into () . unwrap () ,) } _ => MIN_BIG_ARCHIVE_MEM_DATA_ALIGN , } }
};
}
