// Generated macro for get_file_info_ex (function)
macro_rules! Depcrateget_file_info_ex {
() => {
// Module: crate
// Provides: {"get_file_info_ex"}
// Dependencies: {}
# [cfg (target_family = "windows")] unsafe fn get_file_info_ex (file : & fs :: File) -> Result < FileId , io :: Error > { use std :: { mem , os :: windows :: prelude :: * } ; use windows_sys :: Win32 :: { Foundation :: HANDLE , Storage :: FileSystem :: { FileIdInfo , GetFileInformationByHandleEx , FILE_ID_INFO } , } ; let mut info : FILE_ID_INFO = mem :: zeroed () ; let ret = GetFileInformationByHandleEx (file . as_raw_handle () as HANDLE , FileIdInfo , & mut info as * mut FILE_ID_INFO as _ , mem :: size_of :: < FILE_ID_INFO > () as u32 ,) ; if ret == 0 { return Err (io :: Error :: last_os_error ()) ; } ; Ok (FileId :: new_high_res (info . VolumeSerialNumber , u128 :: from_le_bytes (info . FileId . Identifier) ,)) }
};
}
