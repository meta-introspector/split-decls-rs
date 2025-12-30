// Generated macro for macro_25 (macro)
macro_rules! Depcratemacro_25 {
() => {
// Module: crate
// Provides: {"macro_25"}
// Dependencies: {}
if_conversions ! { fn from_raw_bytes <'a , S > (string : S) -> convert :: Result < Cow <'a , OsStr >> where S : Into < Cow <'a , [u8] >>, { match string . into () { Cow :: Borrowed (string) => convert :: os_str_from_bytes (string) , Cow :: Owned (string) => { convert :: os_string_from_vec (string) . map (Cow :: Owned) } } } }
};
}
