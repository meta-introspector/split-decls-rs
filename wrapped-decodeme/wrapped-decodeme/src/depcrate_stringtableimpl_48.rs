// Generated macro for impl_48 (impl)
macro_rules! Depcrate_stringtableimpl_48 {
() => {
// Module: crate::stringtable
// Provides: {"impl_48"}
// Dependencies: {}
impl StringTable { pub fn new (string_data : Vec < u8 > , index_data : Vec < u8 > , diagnostic_file_path : Option < & Path > ,) -> Result < StringTable , Box < dyn Error + Send + Sync > > { verify_file_header (& string_data , FILE_MAGIC_STRINGTABLE_DATA , diagnostic_file_path , "StringTable Data" ,) ? ; verify_file_header (& index_data , FILE_MAGIC_STRINGTABLE_INDEX , diagnostic_file_path , "StringTable Index" ,) ? ; assert ! ((index_data . len () - measureme :: file_header :: FILE_HEADER_SIZE) % INDEX_ENTRY_SIZE == 0 , "StringTable index size appears malformed" ,) ; assert_eq ! (INDEX_ENTRY_SIZE , 16) ; let index : FxHashMap < _ , _ > = strip_file_header (& index_data) . chunks (INDEX_ENTRY_SIZE) . map (deserialize_index_entry) . collect () ; Ok (StringTable { string_data , index }) } # [inline] pub fn get < 'a > (& 'a self , id : StringId) -> StringRef < 'a > { StringRef { id , table : self } } pub fn get_metadata < 'a > (& 'a self) -> StringRef < 'a > { let id = StringId :: new (METADATA_STRING_ID) ; self . get (id) } }
};
}
