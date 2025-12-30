// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl OsStrBytes for OsStr { if_conversions ! { # [inline] fn assert_from_raw_bytes <'a , S > (string : S) -> Cow <'a , Self > where S : Into < Cow <'a , [u8] >>, { expect_encoded ! (from_raw_bytes (string)) } } # [inline] fn from_io_bytes (string : & [u8]) -> Option < & Self > { convert_io :: os_str_from_bytes (string) } if_checked_conversions ! { # [inline] fn from_raw_bytes <'a , S > (string : S) -> Result < Cow <'a , Self >> where S : Into < Cow <'a , [u8] >>, { from_raw_bytes (string) . map_err (EncodingError) } } # [inline] fn to_io_bytes (& self) -> Option < & '_ [u8] > { convert_io :: os_str_to_bytes (self) } # [inline] fn to_io_bytes_lossy (& self) -> Cow < '_ , [u8] > { convert_io :: os_str_to_bytes_lossy (self) } if_conversions ! { # [inline] fn to_raw_bytes (& self) -> Cow <'_ , [u8] > { convert :: os_str_to_bytes (self) } } }
};
}
