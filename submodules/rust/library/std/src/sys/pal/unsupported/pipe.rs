mkuse!{use crate :: fmt ;}
mkuse!{use crate :: io :: { self , BorrowedCursor , IoSlice , IoSliceMut } ;}
mkuse!{use crate :: sys_common :: { FromInner , IntoInner } ;}
mkitem!{mkstruct!{pub struct AnonPipe (!) ;}}
mkitem!{mkimpl!{impl fmt :: Debug for AnonPipe { fn fmt (& self , _ : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 } }}}
mkitem!{mkimpl!{impl AnonPipe { pub fn try_clone (& self) -> io :: Result < Self > { self . 0 } pub fn read (& self , _buf : & mut [u8]) -> io :: Result < usize > { self . 0 } pub fn read_buf (& self , _buf : BorrowedCursor < '_ >) -> io :: Result < () > { self . 0 } pub fn read_vectored (& self , _bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { self . 0 } pub fn is_read_vectored (& self) -> bool { self . 0 } pub fn read_to_end (& self , _buf : & mut Vec < u8 >) -> io :: Result < usize > { self . 0 } pub fn write (& self , _buf : & [u8]) -> io :: Result < usize > { self . 0 } pub fn write_vectored (& self , _bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . 0 } pub fn is_write_vectored (& self) -> bool { self . 0 } pub fn diverge (& self) -> ! { self . 0 } }}}

macro_rules! read2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function read2 in module {}", module_path!());
    };
}

mkfn!{
    read2_introspect!();
    pub fn read2 (p1 : AnonPipe , _v1 : & mut Vec < u8 > , _p2 : AnonPipe , _v2 : & mut Vec < u8 >) -> io :: Result < () > { match p1 . 0 { } }
}
mkitem!{mkimpl!{impl FromInner < ! > for AnonPipe { fn from_inner (inner : !) -> Self { inner } }}}
mkitem!{mkimpl!{impl IntoInner < ! > for AnonPipe { fn into_inner (self) -> ! { self . 0 } }}}
mkmod!{unix_traits, { 
                getname!(unix_traits);
                getsrc!(unix_traits);
                getpath!(unix_traits);
                get_deps!(unix_traits);
                get_crates!(unix_traits);
                mkinclude!(unix_traits);
                mkuse!{use super :: AnonPipe ;}
mkuse!{use crate :: os :: fd :: { AsFd , AsRawFd , BorrowedFd , FromRawFd , IntoRawFd , OwnedFd , RawFd } ;}
mkuse!{use crate :: sys_common :: FromInner ;}
mkitem!{mkimpl!{impl AsRawFd for AnonPipe { # [inline] fn as_raw_fd (& self) -> RawFd { self . 0 } }}}
mkitem!{mkimpl!{impl AsFd for AnonPipe { fn as_fd (& self) -> BorrowedFd < '_ > { self . 0 } }}}
mkitem!{mkimpl!{impl IntoRawFd for AnonPipe { fn into_raw_fd (self) -> RawFd { self . 0 } }}}
mkitem!{mkimpl!{impl FromRawFd for AnonPipe { unsafe fn from_raw_fd (_ : RawFd) -> Self { panic ! ("creating pipe on this platform is unsupported!") } }}}
mkitem!{mkimpl!{impl FromInner < OwnedFd > for AnonPipe { fn from_inner (_ : OwnedFd) -> Self { panic ! ("creating pipe on this platform is unsupported!") } }}} 
            }}