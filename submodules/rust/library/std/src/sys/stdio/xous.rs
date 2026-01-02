mkmod!{unsupported_stdio, { 
                getname!(unsupported_stdio);
                getsrc!(unsupported_stdio);
                getpath!(unsupported_stdio);
                get_deps!(unsupported_stdio);
                get_crates!(unsupported_stdio);
                mkinclude!(unsupported_stdio);
                 
            }}
mkuse!{use crate :: io ;}
mkuse!{use crate :: os :: xous :: ffi :: { Connection , lend , try_lend , try_scalar } ;}
mkuse!{use crate :: os :: xous :: services :: { LogLend , LogScalar , log_server , try_connect } ;}
mkitem!{pub type Stdin = unsupported_stdio :: Stdin ;}
mkitem!{mkstruct!{pub struct Stdout ;}}
mkitem!{mkstruct!{pub struct Stderr ;}}
mkitem!{mkimpl!{impl Stdout { pub const fn new () -> Stdout { Stdout } }}}
mkitem!{mkimpl!{impl io :: Write for Stdout { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { # [repr (C , align (4096))] struct LendBuffer ([u8 ; 4096]) ; let mut lend_buffer = LendBuffer ([0u8 ; 4096]) ; let connection = log_server () ; for chunk in buf . chunks (lend_buffer . 0 . len ()) { for (dest , src) in lend_buffer . 0 . iter_mut () . zip (chunk) { * dest = * src ; } lend (connection , LogLend :: StandardOutput . into () , & lend_buffer . 0 , 0 , chunk . len ()) . unwrap () ; } Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{mkimpl!{impl Stderr { pub const fn new () -> Stderr { Stderr } }}}
mkitem!{mkimpl!{impl io :: Write for Stderr { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { # [repr (C , align (4096))] struct LendBuffer ([u8 ; 4096]) ; let mut lend_buffer = LendBuffer ([0u8 ; 4096]) ; let connection = log_server () ; for chunk in buf . chunks (lend_buffer . 0 . len ()) { for (dest , src) in lend_buffer . 0 . iter_mut () . zip (chunk) { * dest = * src ; } lend (connection , LogLend :: StandardError . into () , & lend_buffer . 0 , 0 , chunk . len ()) . unwrap () ; } Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
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
mkitem!{mkstruct!{# [derive (Copy , Clone)] pub struct PanicWriter { log : Connection , gfx : Option < Connection > , }}}
mkitem!{mkimpl!{impl io :: Write for PanicWriter { fn write (& mut self , s : & [u8]) -> core :: result :: Result < usize , io :: Error > { for c in s . chunks (size_of :: < usize > () * 4) { try_scalar (self . log , LogScalar :: AppendPanicMessage (& c) . into ()) . ok () ; } if let Some (gfx) = self . gfx { # [repr (C , align (4096))] struct Request ([u8 ; 4096]) ; let mut request = Request ([0u8 ; 4096]) ; for (& s , d) in s . iter () . zip (request . 0 . iter_mut ()) { * d = s ; } try_lend (gfx , 0 , & request . 0 , 0 , s . len ()) . ok () ; } Ok (s . len ()) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}

macro_rules! panic_output_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_output in module {}", module_path!());
    };
}

mkfn!{
    panic_output_introspect!();
    pub fn panic_output () -> Option < impl io :: Write > { let log = log_server () ; try_scalar (log , LogScalar :: BeginPanic . into ()) . ok () ; let gfx = try_connect ("panic-to-screen!") ; Some (PanicWriter { log , gfx }) }
}