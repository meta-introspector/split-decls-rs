// Generated macro for windows (module)
macro_rules! Depcrate_filewindows {
() => {
// Module: crate::file
// Provides: {"windows"}
// Dependencies: {}
# [cfg (windows)] mod windows { use crate :: os :: windows :: fs :: FileExt ; use crate :: ErrorKind ; use std :: io ; use std :: os :: windows :: { fs :: FileExt as _ , io :: { AsRawHandle , IntoRawHandle , RawHandle } , } ; impl FileExt for crate :: File { fn seek_read (& self , buf : & mut [u8] , offset : u64) -> io :: Result < usize > { self . file () . seek_read (buf , offset) . map_err (| err | self . error (err , ErrorKind :: SeekRead)) } fn seek_write (& self , buf : & [u8] , offset : u64) -> io :: Result < usize > { self . file () . seek_write (buf , offset) . map_err (| err | self . error (err , ErrorKind :: SeekWrite)) } } impl AsRawHandle for crate :: File { fn as_raw_handle (& self) -> RawHandle { self . file () . as_raw_handle () } } impl IntoRawHandle for crate :: File { fn into_raw_handle (self) -> RawHandle { self . file . into_raw_handle () } } # [cfg (rustc_1_63)] mod io_safety { use std :: os :: windows :: io :: { AsHandle , BorrowedHandle , OwnedHandle } ; impl AsHandle for crate :: File { fn as_handle (& self) -> BorrowedHandle < '_ > { self . file () . as_handle () } } impl From < crate :: File > for OwnedHandle { fn from (file : crate :: File) -> Self { file . into_parts () . 0 . into () } } } }
};
}
