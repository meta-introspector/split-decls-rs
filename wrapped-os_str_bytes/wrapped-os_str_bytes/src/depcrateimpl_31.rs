// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl OsStringBytes for OsString { if_conversions ! { # [inline] fn assert_from_raw_vec (string : Vec < u8 >) -> Self { expect_encoded ! (convert :: os_string_from_vec (string)) } } if_checked_conversions ! { # [inline] fn from_raw_vec (string : Vec < u8 >) -> Result < Self > { convert :: os_string_from_vec (string) . map_err (EncodingError) } } # [inline] fn from_io_vec (string : Vec < u8 >) -> Option < Self > { convert_io :: os_string_from_vec (string) } # [inline] fn into_io_vec (self) -> Option < Vec < u8 > > { convert_io :: os_string_into_vec (self) } # [inline] fn into_io_vec_lossy (self) -> Vec < u8 > { convert_io :: os_string_into_vec_lossy (self) } if_conversions ! { # [inline] fn into_raw_vec (self) -> Vec < u8 > { convert :: os_string_into_vec (self) } } }
};
}
