mkuse!{use super :: usercalls :: alloc :: UserRef ;}
mkuse!{use crate :: io :: { self , Write } ;}
mkuse!{use crate :: { cmp , mem } ;}
mkitem!{unsafe extern "C" { fn take_debug_panic_buf_ptr () -> * mut u8 ; static DEBUG : u8 ; }}
mkitem!{mkstruct!{pub (crate) struct SgxPanicOutput (Option < & 'static mut UserRef < [u8] > >) ;}}

macro_rules! empty_user_slice_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function empty_user_slice in module {}", module_path!());
    };
}

mkfn!{
    empty_user_slice_introspect!();
    fn empty_user_slice () -> & 'static mut UserRef < [u8] > { unsafe { UserRef :: from_raw_parts_mut (1 as * mut u8 , 0) } }
}
mkitem!{mkimpl!{impl SgxPanicOutput { pub (crate) fn new () -> Option < Self > { if unsafe { DEBUG == 0 } { None } else { Some (SgxPanicOutput (None)) } } fn init (& mut self) -> & mut & 'static mut UserRef < [u8] > { self . 0 . get_or_insert_with (| | unsafe { let ptr = take_debug_panic_buf_ptr () ; if ptr . is_null () { empty_user_slice () } else { UserRef :: from_raw_parts_mut (ptr , 1024) } }) } }}}
mkitem!{mkimpl!{impl Write for SgxPanicOutput { fn write (& mut self , src : & [u8]) -> io :: Result < usize > { let dst = mem :: replace (self . init () , empty_user_slice ()) ; let written = cmp :: min (src . len () , dst . len ()) ; dst [.. written] . copy_from_enclave (& src [.. written]) ; self . 0 = Some (& mut dst [written ..]) ; Ok (written) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}