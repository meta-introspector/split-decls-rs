mkuse!{use crate :: io :: { self , BorrowedCursor } ;}
mkuse!{use crate :: sys :: pal :: abi :: { self , fileno } ;}
mkitem!{mkstruct!{pub struct Stdin ;}}
mkitem!{mkstruct!{pub struct Stdout ;}}
mkitem!{mkstruct!{pub struct Stderr ;}}
mkitem!{mkimpl!{impl Stdin { pub const fn new () -> Stdin { Stdin } }}}
mkitem!{mkimpl!{impl io :: Read for Stdin { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { Ok (unsafe { abi :: sys_read (fileno :: STDIN , buf . as_mut_ptr () , buf . len ()) }) } fn read_buf (& mut self , mut buf : BorrowedCursor < '_ >) -> io :: Result < () > { unsafe { let n = abi :: sys_read (fileno :: STDIN , buf . as_mut () . as_mut_ptr () . cast () , buf . capacity ()) ; buf . advance_unchecked (n) ; } Ok (()) } }}}
mkitem!{mkimpl!{impl Stdout { pub const fn new () -> Stdout { Stdout } }}}
mkitem!{mkimpl!{impl io :: Write for Stdout { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { unsafe { abi :: sys_write (fileno :: STDOUT , buf . as_ptr () , buf . len ()) } Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{mkimpl!{impl Stderr { pub const fn new () -> Stderr { Stderr } }}}
mkitem!{mkimpl!{impl io :: Write for Stderr { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { unsafe { abi :: sys_write (fileno :: STDERR , buf . as_ptr () , buf . len ()) } Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{pub const STDIN_BUF_SIZE : usize = crate :: sys :: io :: DEFAULT_BUF_SIZE ;}

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
    pub fn panic_output () -> Option < impl io :: Write > { Some (Stderr :: new ()) }
}