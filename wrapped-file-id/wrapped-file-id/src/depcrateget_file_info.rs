// Generated macro for get_file_info (function)
macro_rules! Depcrateget_file_info {
() => {
// Module: crate
// Provides: {"get_file_info"}
// Dependencies: {}
# [cfg (target_family = "windows")] unsafe fn get_file_info (file : & fs :: File) -> Result < FileId , io :: Error > { use std :: { mem , os :: windows :: prelude :: * } ; use windows_sys :: Win32 :: { Foundation :: HANDLE , Storage :: FileSystem :: { GetFileInformationByHandle , BY_HANDLE_FILE_INFORMATION } , } ; let mut info : BY_HANDLE_FILE_INFORMATION = mem :: zeroed () ; let ret = GetFileInformationByHandle (file . as_raw_handle () as HANDLE , & mut info) ; if ret == 0 { return Err (io :: Error :: last_os_error ()) ; } ; Ok (FileId :: new_low_res (info . dwVolumeSerialNumber , ((info . nFileIndexHigh as u64) << 32) | (info . nFileIndexLow as u64) ,)) }
};
}
