// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl OsStrBytes for Path { if_conversions ! { # [inline] fn assert_from_raw_bytes <'a , S > (string : S) -> Cow <'a , Self > where S : Into < Cow <'a , [u8] >>, { cow_os_str_into_path (OsStr :: assert_from_raw_bytes (string)) } } # [inline] fn from_io_bytes (string : & [u8]) -> Option < & Self > { OsStr :: from_io_bytes (string) . map (Self :: new) } if_checked_conversions ! { # [inline] fn from_raw_bytes <'a , S > (string : S) -> Result < Cow <'a , Self >> where S : Into < Cow <'a , [u8] >>, { OsStr :: from_raw_bytes (string) . map (cow_os_str_into_path) } } # [inline] fn to_io_bytes (& self) -> Option < & '_ [u8] > { self . as_os_str () . to_io_bytes () } # [inline] fn to_io_bytes_lossy (& self) -> Cow < '_ , [u8] > { self . as_os_str () . to_io_bytes_lossy () } if_conversions ! { # [inline] fn to_raw_bytes (& self) -> Cow <'_ , [u8] > { self . as_os_str () . to_raw_bytes () } } }
};
}
