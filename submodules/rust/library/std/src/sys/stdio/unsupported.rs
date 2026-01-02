mkuse!{use crate :: io :: { self , BorrowedCursor , IoSlice , IoSliceMut } ;}
mkitem!{mkstruct!{pub struct Stdin ;}}
mkitem!{mkstruct!{pub struct Stdout ;}}
mkitem!{pub type Stderr = Stdout ;}
mkitem!{mkimpl!{impl Stdin { pub const fn new () -> Stdin { Stdin } }}}
mkitem!{mkimpl!{impl io :: Read for Stdin { # [inline] fn read (& mut self , _buf : & mut [u8]) -> io :: Result < usize > { Ok (0) } # [inline] fn read_buf (& mut self , _cursor : BorrowedCursor < '_ >) -> io :: Result < () > { Ok (()) } # [inline] fn read_vectored (& mut self , _bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { Ok (0) } # [inline] fn is_read_vectored (& self) -> bool { false } # [inline] fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { if ! buf . is_empty () { Err (io :: Error :: READ_EXACT_EOF) } else { Ok (()) } } # [inline] fn read_buf_exact (& mut self , cursor : BorrowedCursor < '_ >) -> io :: Result < () > { if cursor . capacity () != 0 { Err (io :: Error :: READ_EXACT_EOF) } else { Ok (()) } } # [inline] fn read_to_end (& mut self , _buf : & mut Vec < u8 >) -> io :: Result < usize > { Ok (0) } # [inline] fn read_to_string (& mut self , _buf : & mut String) -> io :: Result < usize > { Ok (0) } }}}
mkitem!{mkimpl!{impl Stdout { pub const fn new () -> Stdout { Stdout } }}}
mkitem!{mkimpl!{impl io :: Write for Stdout { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { Ok (buf . len ()) } # [inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { let total_len = bufs . iter () . map (| b | b . len ()) . sum () ; Ok (total_len) } # [inline] fn is_write_vectored (& self) -> bool { true } # [inline] fn write_all (& mut self , _buf : & [u8]) -> io :: Result < () > { Ok (()) } # [inline] fn write_all_vectored (& mut self , _bufs : & mut [IoSlice < '_ >]) -> io :: Result < () > { Ok (()) } # [inline] fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{pub const STDIN_BUF_SIZE : usize = 0 ;}

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
    pub fn panic_output () -> Option < Vec < u8 > > { None }
}