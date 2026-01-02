mkuse!{# [cfg (target_os = "hermit")] use hermit_abi :: { EBADF , STDERR_FILENO , STDIN_FILENO , STDOUT_FILENO } ;}
mkuse!{# [cfg (target_family = "unix")] use libc :: { EBADF , STDERR_FILENO , STDIN_FILENO , STDOUT_FILENO } ;}
mkuse!{use crate :: io :: { self , BorrowedCursor , IoSlice , IoSliceMut } ;}
mkuse!{use crate :: mem :: ManuallyDrop ;}
mkuse!{# [cfg (target_os = "hermit")] use crate :: os :: hermit :: io :: FromRawFd ;}
mkuse!{# [cfg (target_family = "unix")] use crate :: os :: unix :: io :: FromRawFd ;}
mkuse!{use crate :: sys :: fd :: FileDesc ;}
mkitem!{mkstruct!{pub struct Stdin ;}}
mkitem!{mkstruct!{pub struct Stdout ;}}
mkitem!{mkstruct!{pub struct Stderr ;}}
mkitem!{mkimpl!{impl Stdin { pub const fn new () -> Stdin { Stdin } }}}
mkitem!{mkimpl!{impl io :: Read for Stdin { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { unsafe { ManuallyDrop :: new (FileDesc :: from_raw_fd (STDIN_FILENO)) . read (buf) } } fn read_buf (& mut self , buf : BorrowedCursor < '_ >) -> io :: Result < () > { unsafe { ManuallyDrop :: new (FileDesc :: from_raw_fd (STDIN_FILENO)) . read_buf (buf) } } fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { unsafe { ManuallyDrop :: new (FileDesc :: from_raw_fd (STDIN_FILENO)) . read_vectored (bufs) } } # [inline] fn is_read_vectored (& self) -> bool { true } }}}
mkitem!{mkimpl!{impl Stdout { pub const fn new () -> Stdout { Stdout } }}}
mkitem!{mkimpl!{impl io :: Write for Stdout { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { unsafe { ManuallyDrop :: new (FileDesc :: from_raw_fd (STDOUT_FILENO)) . write (buf) } } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { unsafe { ManuallyDrop :: new (FileDesc :: from_raw_fd (STDOUT_FILENO)) . write_vectored (bufs) } } # [inline] fn is_write_vectored (& self) -> bool { true } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{mkimpl!{impl Stderr { pub const fn new () -> Stderr { Stderr } }}}
mkitem!{mkimpl!{impl io :: Write for Stderr { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { unsafe { ManuallyDrop :: new (FileDesc :: from_raw_fd (STDERR_FILENO)) . write (buf) } } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { unsafe { ManuallyDrop :: new (FileDesc :: from_raw_fd (STDERR_FILENO)) . write_vectored (bufs) } } # [inline] fn is_write_vectored (& self) -> bool { true } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}

macro_rules! is_ebadf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_ebadf in module {}", module_path!());
    };
}

mkfn!{
    is_ebadf_introspect!();
    pub fn is_ebadf (err : & io :: Error) -> bool { err . raw_os_error () == Some (EBADF as i32) }
}
mkitem!{pub const STDIN_BUF_SIZE : usize = crate :: sys :: io :: DEFAULT_BUF_SIZE ;}

macro_rules! panic_output_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_output in module {}", module_path!());
    };
}

mkfn!{
    panic_output_introspect!();
    pub fn panic_output () -> Option < impl io :: Write > { Some (Stderr :: new ()) }
}