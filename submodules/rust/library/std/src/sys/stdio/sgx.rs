mkuse!{use fortanix_sgx_abi as abi ;}
mkuse!{use crate :: io :: { self , BorrowedCursor , IoSlice , IoSliceMut } ;}
mkuse!{use crate :: sys :: fd :: FileDesc ;}
mkitem!{mkstruct!{pub struct Stdin ;}}
mkitem!{mkstruct!{pub struct Stdout ;}}
mkitem!{mkstruct!{pub struct Stderr ;}}

macro_rules! with_std_fd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function with_std_fd in module {}", module_path!());
    };
}

mkfn!{
    with_std_fd_introspect!();
    fn with_std_fd < F : FnOnce (& FileDesc) -> R , R > (fd : abi :: Fd , f : F) -> R { let fd = FileDesc :: new (fd) ; let ret = f (& fd) ; fd . into_raw () ; ret }
}
mkitem!{mkimpl!{impl Stdin { pub const fn new () -> Stdin { Stdin } }}}
mkitem!{mkimpl!{impl io :: Read for Stdin { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { with_std_fd (abi :: FD_STDIN , | fd | fd . read (buf)) } fn read_buf (& mut self , buf : BorrowedCursor < '_ >) -> io :: Result < () > { with_std_fd (abi :: FD_STDIN , | fd | fd . read_buf (buf)) } fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { with_std_fd (abi :: FD_STDIN , | fd | fd . read_vectored (bufs)) } # [inline] fn is_read_vectored (& self) -> bool { true } }}}
mkitem!{mkimpl!{impl Stdout { pub const fn new () -> Stdout { Stdout } }}}
mkitem!{mkimpl!{impl io :: Write for Stdout { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { with_std_fd (abi :: FD_STDOUT , | fd | fd . write (buf)) } fn flush (& mut self) -> io :: Result < () > { with_std_fd (abi :: FD_STDOUT , | fd | fd . flush ()) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { with_std_fd (abi :: FD_STDOUT , | fd | fd . write_vectored (bufs)) } # [inline] fn is_write_vectored (& self) -> bool { true } }}}
mkitem!{mkimpl!{impl Stderr { pub const fn new () -> Stderr { Stderr } }}}
mkitem!{mkimpl!{impl io :: Write for Stderr { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { with_std_fd (abi :: FD_STDERR , | fd | fd . write (buf)) } fn flush (& mut self) -> io :: Result < () > { with_std_fd (abi :: FD_STDERR , | fd | fd . flush ()) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { with_std_fd (abi :: FD_STDERR , | fd | fd . write_vectored (bufs)) } # [inline] fn is_write_vectored (& self) -> bool { true } }}}
mkitem!{pub const STDIN_BUF_SIZE : usize = crate :: sys :: io :: DEFAULT_BUF_SIZE ;}

macro_rules! is_ebadf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_ebadf in module {}", module_path!());
    };
}

mkfn!{
    is_ebadf_introspect!();
    pub fn is_ebadf (err : & io :: Error) -> bool { err . raw_os_error () == Some (abi :: Error :: BrokenPipe as _) }
}

macro_rules! panic_output_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_output in module {}", module_path!());
    };
}

mkfn!{
    panic_output_introspect!();
    pub fn panic_output () -> Option < impl io :: Write > { crate :: sys :: pal :: abi :: panic :: SgxPanicOutput :: new () }
}