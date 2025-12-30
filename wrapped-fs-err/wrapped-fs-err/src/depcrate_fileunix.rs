// Generated macro for unix (module)
macro_rules! Depcrate_fileunix {
() => {
// Module: crate::file
// Provides: {"unix"}
// Dependencies: {}
# [cfg (unix)] mod unix { use crate :: os :: unix :: fs :: FileExt ; use crate :: ErrorKind ; use std :: io ; use std :: os :: unix :: fs :: FileExt as _ ; use std :: os :: unix :: io :: { AsRawFd , IntoRawFd , RawFd } ; impl AsRawFd for crate :: File { fn as_raw_fd (& self) -> RawFd { self . file () . as_raw_fd () } } impl IntoRawFd for crate :: File { fn into_raw_fd (self) -> RawFd { self . file . into_raw_fd () } } impl FileExt for crate :: File { fn read_at (& self , buf : & mut [u8] , offset : u64) -> io :: Result < usize > { self . file () . read_at (buf , offset) . map_err (| err | self . error (err , ErrorKind :: ReadAt)) } fn write_at (& self , buf : & [u8] , offset : u64) -> io :: Result < usize > { self . file () . write_at (buf , offset) . map_err (| err | self . error (err , ErrorKind :: WriteAt)) } } # [cfg (rustc_1_63)] mod io_safety { use std :: os :: unix :: io :: { AsFd , BorrowedFd , OwnedFd } ; impl AsFd for crate :: File { fn as_fd (& self) -> BorrowedFd < '_ > { self . file () . as_fd () } } impl From < crate :: File > for OwnedFd { fn from (file : crate :: File) -> Self { file . into_file () . into () } } } }
};
}
