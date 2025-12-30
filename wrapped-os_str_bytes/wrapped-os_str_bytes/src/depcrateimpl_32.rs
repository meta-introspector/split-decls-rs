// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl OsStringBytes for PathBuf { if_conversions ! { # [inline] fn assert_from_raw_vec (string : Vec < u8 >) -> Self { OsString :: assert_from_raw_vec (string) . into () } } if_checked_conversions ! { # [inline] fn from_raw_vec (string : Vec < u8 >) -> Result < Self > { OsString :: from_raw_vec (string) . map (Into :: into) } } # [inline] fn from_io_vec (string : Vec < u8 >) -> Option < Self > { OsString :: from_io_vec (string) . map (Into :: into) } # [inline] fn into_io_vec (self) -> Option < Vec < u8 > > { self . into_os_string () . into_io_vec () } # [inline] fn into_io_vec_lossy (self) -> Vec < u8 > { self . into_os_string () . into_io_vec_lossy () } if_conversions ! { # [inline] fn into_raw_vec (self) -> Vec < u8 > { self . into_os_string () . into_raw_vec () } } }
};
}
