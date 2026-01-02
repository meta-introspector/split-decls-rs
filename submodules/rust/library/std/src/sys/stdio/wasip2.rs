mkuse!{use wasip2 :: cli ;}
mkuse!{use wasip2 :: io :: streams :: { Error , InputStream , OutputStream , StreamError } ;}
mkuse!{use crate :: io :: { self , BorrowedBuf , BorrowedCursor } ;}
mkitem!{mkstruct!{pub struct Stdin (Option < InputStream >) ;}}
mkitem!{mkstruct!{pub struct Stdout (Option < OutputStream >) ;}}
mkitem!{mkstruct!{pub struct Stderr (Option < OutputStream >) ;}}

macro_rules! error_to_io_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function error_to_io in module {}", module_path!());
    };
}

mkfn!{
    error_to_io_introspect!();
    fn error_to_io (err : Error) -> io :: Error { io :: Error :: new (io :: ErrorKind :: Other , err . to_debug_string ()) }
}
mkitem!{mkimpl!{impl Stdin { pub const fn new () -> Stdin { Stdin (None) } fn stream (& mut self) -> & InputStream { self . 0 . get_or_insert_with (cli :: stdin :: get_stdin) } }}}
mkitem!{mkimpl!{impl io :: Read for Stdin { fn read (& mut self , data : & mut [u8]) -> io :: Result < usize > { let mut buf = BorrowedBuf :: from (data) ; self . read_buf (buf . unfilled ()) ? ; Ok (buf . len ()) } fn read_buf (& mut self , mut buf : BorrowedCursor < '_ >) -> io :: Result < () > { match self . stream () . blocking_read (u64 :: try_from (buf . capacity ()) . unwrap ()) { Ok (result) => { buf . append (& result) ; Ok (()) } Err (StreamError :: Closed) => Ok (()) , Err (StreamError :: LastOperationFailed (e)) => Err (error_to_io (e)) , } } }}}
mkitem!{mkimpl!{impl Stdout { pub const fn new () -> Stdout { Stdout (None) } fn stream (& mut self) -> & OutputStream { self . 0 . get_or_insert_with (cli :: stdout :: get_stdout) } }}}

macro_rules! write_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write in module {}", module_path!());
    };
}

mkfn!{
    write_introspect!();
    fn write (stream : & OutputStream , buf : & [u8]) -> io :: Result < usize > { const MAX : usize = 4096 ; let buf = & buf [.. buf . len () . min (MAX)] ; match stream . blocking_write_and_flush (buf) { Ok (()) => Ok (buf . len ()) , Err (StreamError :: Closed) => Ok (0) , Err (StreamError :: LastOperationFailed (e)) => Err (error_to_io (e)) , } }
}
mkitem!{mkimpl!{impl io :: Write for Stdout { fn write (& mut self , data : & [u8]) -> io :: Result < usize > { write (self . stream () , data) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{mkimpl!{impl Stderr { pub const fn new () -> Stderr { Stderr (None) } fn stream (& mut self) -> & OutputStream { self . 0 . get_or_insert_with (cli :: stderr :: get_stderr) } }}}
mkitem!{mkimpl!{impl io :: Write for Stderr { fn write (& mut self , data : & [u8]) -> io :: Result < usize > { write (self . stream () , data) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{pub const STDIN_BUF_SIZE : usize = crate :: sys :: io :: DEFAULT_BUF_SIZE ;}

macro_rules! is_ebadf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_ebadf in module {}", module_path!());
    };
}

mkfn!{
    is_ebadf_introspect!();
    pub fn is_ebadf (_err : & io :: Error) -> bool { false }
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