mkuse!{use crate :: io :: { self , BorrowedCursor , IoSlice , IoSliceMut } ;}
mkuse!{use crate :: mem ;}
mkuse!{use crate :: os :: unix :: io :: { AsFd , AsRawFd , BorrowedFd , FromRawFd , IntoRawFd , RawFd } ;}
mkuse!{use crate :: sys :: fd :: FileDesc ;}
mkuse!{use crate :: sys :: { cvt , cvt_r } ;}
mkuse!{use crate :: sys_common :: { FromInner , IntoInner } ;}
mkitem!{mkstruct!{# [derive (Debug)] pub struct AnonPipe (FileDesc) ;}}

macro_rules! anon_pipe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function anon_pipe in module {}", module_path!());
    };
}

mkfn!{
    anon_pipe_introspect!();
    pub fn anon_pipe () -> io :: Result < (AnonPipe , AnonPipe) > { let mut fds = [0 ; 2] ; cfg_select ! { any (target_os = "android" , target_os = "dragonfly" , target_os = "freebsd" , target_os = "hurd" , target_os = "illumos" , target_os = "linux" , target_os = "netbsd" , target_os = "openbsd" , target_os = "cygwin" , target_os = "redox") => { unsafe { cvt (libc :: pipe2 (fds . as_mut_ptr () , libc :: O_CLOEXEC)) ?; Ok ((AnonPipe (FileDesc :: from_raw_fd (fds [0])) , AnonPipe (FileDesc :: from_raw_fd (fds [1])))) } } _ => { unsafe { cvt (libc :: pipe (fds . as_mut_ptr ())) ?; let fd0 = FileDesc :: from_raw_fd (fds [0]) ; let fd1 = FileDesc :: from_raw_fd (fds [1]) ; fd0 . set_cloexec () ?; fd1 . set_cloexec () ?; Ok ((AnonPipe (fd0) , AnonPipe (fd1))) } } } }
}
mkitem!{mkimpl!{impl AnonPipe { # [allow (dead_code)] pub fn try_clone (& self) -> io :: Result < Self > { self . 0 . duplicate () . map (Self) } pub fn read (& self , buf : & mut [u8]) -> io :: Result < usize > { self . 0 . read (buf) } pub fn read_buf (& self , buf : BorrowedCursor < '_ >) -> io :: Result < () > { self . 0 . read_buf (buf) } pub fn read_vectored (& self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { self . 0 . read_vectored (bufs) } # [inline] pub fn is_read_vectored (& self) -> bool { self . 0 . is_read_vectored () } pub fn read_to_end (& self , buf : & mut Vec < u8 >) -> io :: Result < usize > { self . 0 . read_to_end (buf) } pub fn write (& self , buf : & [u8]) -> io :: Result < usize > { self . 0 . write (buf) } pub fn write_vectored (& self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . 0 . write_vectored (bufs) } # [inline] pub fn is_write_vectored (& self) -> bool { self . 0 . is_write_vectored () } # [allow (dead_code)] pub fn as_file_desc (& self) -> & FileDesc { & self . 0 } }}}
mkitem!{mkimpl!{impl IntoInner < FileDesc > for AnonPipe { fn into_inner (self) -> FileDesc { self . 0 } }}}

macro_rules! read2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function read2 in module {}", module_path!());
    };
}

mkfn!{
    read2_introspect!();
    pub fn read2 (p1 : AnonPipe , v1 : & mut Vec < u8 > , p2 : AnonPipe , v2 : & mut Vec < u8 >) -> io :: Result < () > { let p1 = p1 . into_inner () ; let p2 = p2 . into_inner () ; p1 . set_nonblocking (true) ? ; p2 . set_nonblocking (true) ? ; let mut fds : [libc :: pollfd ; 2] = unsafe { mem :: zeroed () } ; fds [0] . fd = p1 . as_raw_fd () ; fds [0] . events = libc :: POLLIN ; fds [1] . fd = p2 . as_raw_fd () ; fds [1] . events = libc :: POLLIN ; loop { cvt_r (| | unsafe { libc :: poll (fds . as_mut_ptr () , 2 , - 1) }) ? ; if fds [0] . revents != 0 && read (& p1 , v1) ? { p2 . set_nonblocking (false) ? ; return p2 . read_to_end (v2) . map (drop) ; } if fds [1] . revents != 0 && read (& p2 , v2) ? { p1 . set_nonblocking (false) ? ; return p1 . read_to_end (v1) . map (drop) ; } } fn read (fd : & FileDesc , dst : & mut Vec < u8 >) -> Result < bool , io :: Error > { match fd . read_to_end (dst) { Ok (_) => Ok (true) , Err (e) => { if e . raw_os_error () == Some (libc :: EWOULDBLOCK) || e . raw_os_error () == Some (libc :: EAGAIN) { Ok (false) } else { Err (e) } } } } }
}
mkitem!{mkimpl!{impl AsRawFd for AnonPipe { # [inline] fn as_raw_fd (& self) -> RawFd { self . 0 . as_raw_fd () } }}}
mkitem!{mkimpl!{impl AsFd for AnonPipe { fn as_fd (& self) -> BorrowedFd < '_ > { self . 0 . as_fd () } }}}
mkitem!{mkimpl!{impl IntoRawFd for AnonPipe { fn into_raw_fd (self) -> RawFd { self . 0 . into_raw_fd () } }}}
mkitem!{mkimpl!{impl FromRawFd for AnonPipe { unsafe fn from_raw_fd (raw_fd : RawFd) -> Self { Self (FromRawFd :: from_raw_fd (raw_fd)) } }}}
mkitem!{mkimpl!{impl FromInner < FileDesc > for AnonPipe { fn from_inner (fd : FileDesc) -> Self { Self (fd) } }}}