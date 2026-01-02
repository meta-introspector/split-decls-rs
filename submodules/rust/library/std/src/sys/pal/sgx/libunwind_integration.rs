mkuse!{use crate :: sys :: sync :: RwLock ;}
mkuse!{use crate :: { slice , str } ;}
mkitem!{const _ : () = unsafe { let bits_rust : usize = crate :: mem :: transmute (RwLock :: new ()) ; assert ! (bits_rust == 0) ; } ;}
mkitem!{const EINVAL : i32 = 22 ;}

macro_rules! __rust_rwlock_rdlock_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __rust_rwlock_rdlock in module {}", module_path!());
    };
}

mkfn!{
    __rust_rwlock_rdlock_introspect!();
    # [unsafe (no_mangle)] pub unsafe extern "C" fn __rust_rwlock_rdlock (p : * mut RwLock) -> i32 { if p . is_null () { return EINVAL ; } unsafe { (* p) . write () } ; return 0 ; }
}

macro_rules! __rust_rwlock_wrlock_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __rust_rwlock_wrlock in module {}", module_path!());
    };
}

mkfn!{
    __rust_rwlock_wrlock_introspect!();
    # [unsafe (no_mangle)] pub unsafe extern "C" fn __rust_rwlock_wrlock (p : * mut RwLock) -> i32 { if p . is_null () { return EINVAL ; } unsafe { (* p) . write () } ; return 0 ; }
}

macro_rules! __rust_rwlock_unlock_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __rust_rwlock_unlock in module {}", module_path!());
    };
}

mkfn!{
    __rust_rwlock_unlock_introspect!();
    # [unsafe (no_mangle)] pub unsafe extern "C" fn __rust_rwlock_unlock (p : * mut RwLock) -> i32 { if p . is_null () { return EINVAL ; } unsafe { (* p) . write_unlock () } ; return 0 ; }
}

macro_rules! __rust_print_err_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __rust_print_err in module {}", module_path!());
    };
}

mkfn!{
    __rust_print_err_introspect!();
    # [unsafe (no_mangle)] pub unsafe extern "C" fn __rust_print_err (m : * mut u8 , s : i32) { if s < 0 { return ; } let buf = unsafe { slice :: from_raw_parts (m as * const u8 , s as _) } ; if let Ok (s) = str :: from_utf8 (& buf [.. buf . iter () . position (| & b | b == 0) . unwrap_or (buf . len ())]) { eprint ! ("{s}") ; } }
}