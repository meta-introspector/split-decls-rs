mkuse!{use crate :: sys :: pal :: abi :: tls :: { Key as AbiKey , Tls } ;}
mkitem!{pub type Key = usize ;}

macro_rules! create_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create in module {}", module_path!());
    };
}

mkfn!{
    create_introspect!();
    # [inline] pub fn create (dtor : Option < unsafe extern "C" fn (* mut u8) >) -> Key { Tls :: create (dtor) . as_usize () }
}

macro_rules! set_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set in module {}", module_path!());
    };
}

mkfn!{
    set_introspect!();
    # [inline] pub unsafe fn set (key : Key , value : * mut u8) { Tls :: set (AbiKey :: from_usize (key) , value) }
}

macro_rules! get_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get in module {}", module_path!());
    };
}

mkfn!{
    get_introspect!();
    # [inline] pub unsafe fn get (key : Key) -> * mut u8 { Tls :: get (AbiKey :: from_usize (key)) }
}

macro_rules! destroy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function destroy in module {}", module_path!());
    };
}

mkfn!{
    destroy_introspect!();
    # [inline] pub unsafe fn destroy (key : Key) { Tls :: destroy (AbiKey :: from_usize (key)) }
}