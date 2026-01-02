mkuse!{use crate :: io :: { self , BorrowedCursor , IoSlice , IoSliceMut } ;}
mkuse!{use crate :: mem :: ManuallyDrop ;}
mkuse!{use crate :: os :: raw ;}
mkuse!{use crate :: os :: wasi :: io :: { AsRawFd , FromRawFd } ;}
mkuse!{use crate :: sys :: fd :: WasiFd ;}
mkitem!{mkstruct!{pub struct Stdin ;}}
mkitem!{mkstruct!{pub struct Stdout ;}}
mkitem!{mkstruct!{pub struct Stderr ;}}
mkitem!{mkimpl!{impl Stdin { pub const fn new () -> Stdin { Stdin } }}}
mkitem!{mkimpl!{impl AsRawFd for Stdin { # [inline] fn as_raw_fd (& self) -> raw :: c_int { 0 } }}}
mkitem!{mkimpl!{impl io :: Read for Stdin { fn read (& mut self , data : & mut [u8]) -> io :: Result < usize > { self . read_vectored (& mut [IoSliceMut :: new (data)]) } fn read_buf (& mut self , buf : BorrowedCursor < '_ >) -> io :: Result < () > { ManuallyDrop :: new (unsafe { WasiFd :: from_raw_fd (self . as_raw_fd ()) }) . read_buf (buf) } fn read_vectored (& mut self , data : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { ManuallyDrop :: new (unsafe { WasiFd :: from_raw_fd (self . as_raw_fd ()) }) . read (data) } # [inline] fn is_read_vectored (& self) -> bool { true } }}}
mkitem!{mkimpl!{impl Stdout { pub const fn new () -> Stdout { Stdout } }}}
mkitem!{mkimpl!{impl AsRawFd for Stdout { # [inline] fn as_raw_fd (& self) -> raw :: c_int { 1 } }}}
mkitem!{mkimpl!{impl io :: Write for Stdout { fn write (& mut self , data : & [u8]) -> io :: Result < usize > { self . write_vectored (& [IoSlice :: new (data)]) } fn write_vectored (& mut self , data : & [IoSlice < '_ >]) -> io :: Result < usize > { ManuallyDrop :: new (unsafe { WasiFd :: from_raw_fd (self . as_raw_fd ()) }) . write (data) } # [inline] fn is_write_vectored (& self) -> bool { true } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{mkimpl!{impl Stderr { pub const fn new () -> Stderr { Stderr } }}}
mkitem!{mkimpl!{impl AsRawFd for Stderr { # [inline] fn as_raw_fd (& self) -> raw :: c_int { 2 } }}}
mkitem!{mkimpl!{impl io :: Write for Stderr { fn write (& mut self , data : & [u8]) -> io :: Result < usize > { self . write_vectored (& [IoSlice :: new (data)]) } fn write_vectored (& mut self , data : & [IoSlice < '_ >]) -> io :: Result < usize > { ManuallyDrop :: new (unsafe { WasiFd :: from_raw_fd (self . as_raw_fd ()) }) . write (data) } # [inline] fn is_write_vectored (& self) -> bool { true } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{pub const STDIN_BUF_SIZE : usize = crate :: sys :: io :: DEFAULT_BUF_SIZE ;}

macro_rules! is_ebadf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_ebadf in module {}", module_path!());
    };
}

mkfn!{
    is_ebadf_introspect!();
    pub fn is_ebadf (err : & io :: Error) -> bool { err . raw_os_error () == Some (wasi :: ERRNO_BADF . raw () . into ()) }
}

macro_rules! panic_output_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_output in module {}", module_path!());
    };
}

mkfn!{
    panic_output_introspect!();
    pub fn panic_output () -> Option < impl io :: Write > { Some (Stderr :: new ()) }
}