mkmod!{unsupported_stdio, { 
                getname!(unsupported_stdio);
                getsrc!(unsupported_stdio);
                getpath!(unsupported_stdio);
                get_deps!(unsupported_stdio);
                get_crates!(unsupported_stdio);
                mkinclude!(unsupported_stdio);
                 
            }}
mkuse!{use crate :: io ;}
mkuse!{use crate :: sys :: pal :: abi ;}
mkitem!{pub type Stdin = unsupported_stdio :: Stdin ;}
mkitem!{mkstruct!{pub struct Stdout ;}}
mkitem!{pub type Stderr = Stdout ;}
mkitem!{mkimpl!{impl Stdout { pub const fn new () -> Stdout { Stdout } }}}
mkitem!{mkimpl!{impl io :: Write for Stdout { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { unsafe { abi :: SOLID_LOG_write (buf . as_ptr () , buf . len ()) } ; Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
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
    pub fn panic_output () -> Option < impl io :: Write > { Some (Stderr :: new ()) }
}