// Generated macro for compute_string_table (function)
macro_rules! Depcrate_archive_writercompute_string_table {
() => {
// Module: crate::archive_writer
// Provides: {"compute_string_table"}
// Dependencies: {}
fn compute_string_table (names : & [u8]) -> MemberData < '_ > { let size = u64 :: try_from (names . len ()) . unwrap () ; let pad = offset_to_alignment (size , 2) ; let mut header = Vec :: new () ; write ! (header , "{:<48}" , "//") . unwrap () ; write ! (header , "{:<10}" , size + pad) . unwrap () ; write ! (header , "`\n") . unwrap () ; MemberData { symbols : vec ! [] , header , data : names , padding : if pad != 0 { b"\n" } else { b"" } , pre_head_pad_size : 0 , object_reader : & crate :: DEFAULT_OBJECT_READER , } }
};
}
