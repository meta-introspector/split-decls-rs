mkuse!{use crate :: mem ;}
mkmod!{libc, { 
                getname!(libc);
                getsrc!(libc);
                getpath!(libc);
                get_deps!(libc);
                get_crates!(libc);
                mkinclude!(libc);
                mkuse!{use crate :: ffi ;}
mkitem!{# [allow (non_camel_case_types)] pub type pthread_key_t = ffi :: c_uint ;}
mkitem!{unsafe extern "C" { pub fn pthread_key_create (key : * mut pthread_key_t , destructor : unsafe extern "C" fn (* mut ffi :: c_void) ,) -> ffi :: c_int ; # [allow (dead_code)] pub fn pthread_getspecific (key : pthread_key_t) -> * mut ffi :: c_void ; pub fn pthread_setspecific (key : pthread_key_t , value : * const ffi :: c_void) -> ffi :: c_int ; pub fn pthread_key_delete (key : pthread_key_t) -> ffi :: c_int ; }} 
            }}
mkitem!{pub type Key = libc :: pthread_key_t ;}

macro_rules! create_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create in module {}", module_path!());
    };
}

mkfn!{
    create_introspect!();
    # [inline] pub fn create (dtor : Option < unsafe extern "C" fn (* mut u8) >) -> Key { let mut key = 0 ; if unsafe { libc :: pthread_key_create (& mut key , mem :: transmute (dtor)) } != 0 { rtabort ! ("out of TLS keys") ; } key }
}

macro_rules! set_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set in module {}", module_path!());
    };
}

mkfn!{
    set_introspect!();
    # [inline] pub unsafe fn set (key : Key , value : * mut u8) { let r = unsafe { libc :: pthread_setspecific (key , value as * mut _) } ; debug_assert_eq ! (r , 0) ; }
}

macro_rules! get_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get in module {}", module_path!());
    };
}

mkfn!{
    get_introspect!();
    # [inline] # [cfg (any (not (target_thread_local) , test))] pub unsafe fn get (key : Key) -> * mut u8 { unsafe { libc :: pthread_getspecific (key) as * mut u8 } }
}

macro_rules! destroy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function destroy in module {}", module_path!());
    };
}

mkfn!{
    destroy_introspect!();
    # [inline] pub unsafe fn destroy (key : Key) { let r = unsafe { libc :: pthread_key_delete (key) } ; debug_assert_eq ! (r , 0) ; }
}