mkmod!{unsupported_stdio, { 
                getname!(unsupported_stdio);
                getsrc!(unsupported_stdio);
                getpath!(unsupported_stdio);
                get_deps!(unsupported_stdio);
                get_crates!(unsupported_stdio);
                mkinclude!(unsupported_stdio);
                 
            }}
mkuse!{use crate :: cmp ;}
mkuse!{use crate :: io :: { self , IoSlice } ;}
mkitem!{pub type Stdin = unsupported_stdio :: Stdin ;}
mkitem!{mkstruct!{pub struct Stdout ;}}
mkitem!{mkstruct!{pub struct Stderr ;}}
mkitem!{mkimpl!{impl Stdout { pub const fn new () -> Stdout { Stdout } }}}
mkitem!{mkimpl!{impl io :: Write for Stdout { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { write (libc :: STDOUT_FILENO , buf) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { write_vectored (libc :: STDOUT_FILENO , bufs) } # [inline] fn is_write_vectored (& self) -> bool { true } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{mkimpl!{impl Stderr { pub const fn new () -> Stderr { Stderr } }}}
mkitem!{mkimpl!{impl io :: Write for Stderr { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { write (libc :: STDERR_FILENO , buf) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { write_vectored (libc :: STDERR_FILENO , bufs) } # [inline] fn is_write_vectored (& self) -> bool { true } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{pub const STDIN_BUF_SIZE : usize = unsupported_stdio :: STDIN_BUF_SIZE ;}

macro_rules! is_ebadf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_ebadf in module {}", module_path!());
    };
}

mkfn!{
    is_ebadf_introspect!();
    pub fn is_ebadf (_err : & io :: Error) -> bool { true }
}

macro_rules! panic_output_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_output in module {}", module_path!());
    };
}

mkfn!{
    panic_output_introspect!();
    pub fn panic_output () -> Option < impl io :: Write > { Some (Stderr) }
}

macro_rules! write_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write in module {}", module_path!());
    };
}

mkfn!{
    write_introspect!();
    fn write (fd : i32 , buf : & [u8]) -> io :: Result < usize > { let iov = libc :: iovec { iov_base : buf . as_ptr () as * mut _ , iov_len : buf . len () } ; let ret = unsafe { libc :: writev (fd , & iov , 1) } ; if ret as usize > iov . iov_len { return Err (io :: Error :: last_os_error ()) ; } Ok (ret as usize) }
}

macro_rules! write_vectored_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_vectored in module {}", module_path!());
    };
}

mkfn!{
    write_vectored_introspect!();
    fn write_vectored (fd : i32 , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { let iov = bufs . as_ptr () as * const libc :: iovec ; let len = cmp :: min (bufs . len () , libc :: c_int :: MAX as usize) as libc :: c_int ; let ret = unsafe { libc :: writev (fd , iov , len) } ; if ret < 0 { return Err (io :: Error :: last_os_error ()) ; } Ok (ret as usize) }
}