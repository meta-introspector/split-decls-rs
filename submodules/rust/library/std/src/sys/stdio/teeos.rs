mkmod!{unsupported_stdio, { 
                getname!(unsupported_stdio);
                getsrc!(unsupported_stdio);
                getpath!(unsupported_stdio);
                get_deps!(unsupported_stdio);
                get_crates!(unsupported_stdio);
                mkinclude!(unsupported_stdio);
                 
            }}
mkuse!{use core :: arch :: asm ;}
mkuse!{use crate :: io ;}
mkitem!{pub type Stdin = unsupported_stdio :: Stdin ;}
mkitem!{mkstruct!{pub struct Stdout ;}}
mkitem!{pub type Stderr = Stdout ;}
mkitem!{const KCALL_DEBUG_CMD_PUT_BYTES : i64 = 2 ;}

macro_rules! debug_call_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function debug_call in module {}", module_path!());
    };
}

mkfn!{
    debug_call_introspect!();
    unsafe fn debug_call (cap_ref : u64 , call_no : i64 , arg1 : u64 , arg2 : u64) -> i32 { let ret : u64 ; unsafe { asm ! ("svc #99" , inout ("x0") cap_ref => ret , in ("x1") call_no , in ("x2") arg1 , in ("x3") arg2 ,) ; } ret as i32 }
}
mkitem!{mkimpl!{impl Stdout { pub const fn new () -> Stdout { Stdout } }}}
mkitem!{mkimpl!{impl io :: Write for Stdout { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { const MAX_LEN : usize = 512 ; let len = buf . len () . min (MAX_LEN) ; let result = unsafe { debug_call (0 , KCALL_DEBUG_CMD_PUT_BYTES , buf . as_ptr () as u64 , len as u64) } ; if result == 0 { Ok (len) } else { Err (io :: Error :: from (io :: ErrorKind :: InvalidInput)) } } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{pub const STDIN_BUF_SIZE : usize = unsupported_stdio :: STDIN_BUF_SIZE ;}

macro_rules! is_ebadf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_ebadf in module {}", module_path!());
    };
}

mkfn!{
    is_ebadf_introspect!();
    pub fn is_ebadf (err : & io :: Error) -> bool { err . raw_os_error () == Some (libc :: EBADF as i32) }
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