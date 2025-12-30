// Generated macro for tests (module)
macro_rules! Depcrate_file_headertests {
() => {
// Module: crate::file_header
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: { PageTag , SerializationSinkBuilder } ; # [test] fn roundtrip () { let data_sink = SerializationSinkBuilder :: new_in_memory () . new_sink (PageTag :: Events) ; write_file_header (& mut data_sink . as_std_write () , FILE_MAGIC_EVENT_STREAM) . unwrap () ; let data = data_sink . into_bytes () ; verify_file_header (& data , FILE_MAGIC_EVENT_STREAM , None , "test") . unwrap () ; } # [test] fn invalid_magic () { let data_sink = SerializationSinkBuilder :: new_in_memory () . new_sink (PageTag :: Events) ; write_file_header (& mut data_sink . as_std_write () , FILE_MAGIC_STRINGTABLE_DATA) . unwrap () ; let mut data = data_sink . into_bytes () ; data [2] = 0 ; assert ! (verify_file_header (& data , FILE_MAGIC_STRINGTABLE_DATA , None , "test") . is_err ()) ; } # [test] fn other_version () { let data_sink = SerializationSinkBuilder :: new_in_memory () . new_sink (PageTag :: Events) ; write_file_header (& mut data_sink . as_std_write () , FILE_MAGIC_STRINGTABLE_INDEX) . unwrap () ; let mut data = data_sink . into_bytes () ; data [4] = 0xFF ; data [5] = 0xFF ; data [6] = 0xFF ; data [7] = 0xFF ; assert ! (verify_file_header (& data , FILE_MAGIC_STRINGTABLE_INDEX , None , "test") . is_err ()) ; } # [test] fn empty_file () { let data : [u8 ; 0] = [] ; assert ! (verify_file_header (& data , FILE_MAGIC_STRINGTABLE_DATA , None , "test") . is_err ()) ; } }
};
}
