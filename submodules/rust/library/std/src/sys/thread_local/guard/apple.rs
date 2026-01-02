mkuse!{use crate :: cell :: Cell ;}
mkuse!{use crate :: ptr ;}
mkuse!{use crate :: sys :: thread_local :: destructors ;}

macro_rules! enable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function enable in module {}", module_path!());
    };
}

mkfn!{
    enable_introspect!();
    pub fn enable () { # [thread_local] static REGISTERED : Cell < bool > = Cell :: new (false) ; unsafe extern "C" { fn _tlv_atexit (dtor : unsafe extern "C" fn (* mut u8) , arg : * mut u8) ; } if ! REGISTERED . replace (true) { unsafe { _tlv_atexit (run_dtors , ptr :: null_mut ()) ; } } unsafe extern "C" fn run_dtors (_ : * mut u8) { unsafe { destructors :: run () ; crate :: rt :: thread_cleanup () ; } } }
}