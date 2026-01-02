mkuse!{use fortanix_sgx_abi :: Fd ;}
mkuse!{use crate :: io :: { self , BorrowedCursor , IoSlice , IoSliceMut } ;}
mkuse!{use crate :: mem :: ManuallyDrop ;}
mkuse!{use crate :: sys :: pal :: abi :: usercalls ;}
mkuse!{use crate :: sys :: { AsInner , FromInner , IntoInner } ;}
mkitem!{mkstruct!{# [derive (Debug)] pub struct FileDesc { fd : Fd , }}}
mkitem!{mkimpl!{impl FileDesc { pub fn new (fd : Fd) -> FileDesc { FileDesc { fd } } pub fn raw (& self) -> Fd { self . fd } # [doc = " Extracts the actual file descriptor without closing it."] pub fn into_raw (self) -> Fd { ManuallyDrop :: new (self) . fd } pub fn read (& self , buf : & mut [u8]) -> io :: Result < usize > { usercalls :: read (self . fd , & mut [IoSliceMut :: new (buf)]) } pub fn read_buf (& self , buf : BorrowedCursor < '_ >) -> io :: Result < () > { usercalls :: read_buf (self . fd , buf) } pub fn read_vectored (& self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { usercalls :: read (self . fd , bufs) } # [inline] pub fn is_read_vectored (& self) -> bool { true } pub fn write (& self , buf : & [u8]) -> io :: Result < usize > { usercalls :: write (self . fd , & [IoSlice :: new (buf)]) } pub fn write_vectored (& self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { usercalls :: write (self . fd , bufs) } # [inline] pub fn is_write_vectored (& self) -> bool { true } pub fn flush (& self) -> io :: Result < () > { usercalls :: flush (self . fd) } }}}
mkitem!{mkimpl!{impl AsInner < Fd > for FileDesc { # [inline] fn as_inner (& self) -> & Fd { & self . fd } }}}
mkitem!{mkimpl!{impl IntoInner < Fd > for FileDesc { fn into_inner (self) -> Fd { ManuallyDrop :: new (self) . fd } }}}
mkitem!{mkimpl!{impl FromInner < Fd > for FileDesc { fn from_inner (fd : Fd) -> FileDesc { FileDesc { fd } } }}}
mkitem!{mkimpl!{impl Drop for FileDesc { fn drop (& mut self) { usercalls :: close (self . fd) } }}}