mkuse!{pub use super :: common :: Env ;}
mkuse!{use crate :: ffi :: { OsStr , OsString } ;}
mkuse!{use crate :: io ;}

macro_rules! env_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function env in module {}", module_path!());
    };
}

mkfn!{
    env_introspect!();
    pub fn env () -> Env { let env = uefi_env :: get_all () . expect ("not supported on this platform") ; Env :: new (env) }
}

macro_rules! getenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getenv in module {}", module_path!());
    };
}

mkfn!{
    getenv_introspect!();
    pub fn getenv (key : & OsStr) -> Option < OsString > { uefi_env :: get (key) }
}

macro_rules! setenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function setenv in module {}", module_path!());
    };
}

mkfn!{
    setenv_introspect!();
    pub unsafe fn setenv (key : & OsStr , val : & OsStr) -> io :: Result < () > { uefi_env :: set (key , val) }
}

macro_rules! unsetenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unsetenv in module {}", module_path!());
    };
}

mkfn!{
    unsetenv_introspect!();
    pub unsafe fn unsetenv (key : & OsStr) -> io :: Result < () > { uefi_env :: unset (key) }
}
mkmod!{uefi_env, { 
                getname!(uefi_env);
                getsrc!(uefi_env);
                getpath!(uefi_env);
                get_deps!(uefi_env);
                get_crates!(uefi_env);
                mkinclude!(uefi_env);
                mkuse!{use crate :: ffi :: { OsStr , OsString } ;}
mkuse!{use crate :: io ;}
mkuse!{use crate :: os :: uefi :: ffi :: OsStringExt ;}
mkuse!{use crate :: ptr :: NonNull ;}
mkuse!{use crate :: sys :: { helpers , unsupported_err } ;}

macro_rules! get_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get in module {}", module_path!());
    };
}

mkfn!{
    get_introspect!();
    pub (crate) fn get (key : & OsStr) -> Option < OsString > { let shell = helpers :: open_shell () ? ; let mut key_ptr = helpers :: os_string_to_raw (key) ? ; unsafe { get_raw (shell , key_ptr . as_mut_ptr ()) } }
}

macro_rules! set_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set in module {}", module_path!());
    };
}

mkfn!{
    set_introspect!();
    pub (crate) fn set (key : & OsStr , val : & OsStr) -> io :: Result < () > { let mut key_ptr = helpers :: os_string_to_raw (key) . ok_or (io :: const_error ! (io :: ErrorKind :: InvalidInput , "invalid key")) ? ; let mut val_ptr = helpers :: os_string_to_raw (val) . ok_or (io :: const_error ! (io :: ErrorKind :: InvalidInput , "invalid value")) ? ; unsafe { set_raw (key_ptr . as_mut_ptr () , val_ptr . as_mut_ptr ()) } }
}

macro_rules! unset_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unset in module {}", module_path!());
    };
}

mkfn!{
    unset_introspect!();
    pub (crate) fn unset (key : & OsStr) -> io :: Result < () > { let mut key_ptr = helpers :: os_string_to_raw (key) . ok_or (io :: const_error ! (io :: ErrorKind :: InvalidInput , "invalid key")) ? ; unsafe { set_raw (key_ptr . as_mut_ptr () , crate :: ptr :: null_mut ()) } }
}

macro_rules! get_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_all in module {}", module_path!());
    };
}

mkfn!{
    get_all_introspect!();
    pub (crate) fn get_all () -> io :: Result < Vec < (OsString , OsString) > > { let shell = helpers :: open_shell () . ok_or (unsupported_err ()) ? ; let mut vars = Vec :: new () ; let val = unsafe { ((* shell . as_ptr ()) . get_env) (crate :: ptr :: null_mut ()) } ; if val . is_null () { return Ok (vars) ; } let mut start = 0 ; for i in 0 .. { if unsafe { * val . add (i) } == 0 { if i == start { break ; } let key = OsString :: from_wide (unsafe { crate :: slice :: from_raw_parts (val . add (start) , i - start) }) ; let val = unsafe { get_raw (shell , val . add (start)) } . ok_or (io :: const_error ! (io :: ErrorKind :: InvalidInput , "invalid value")) ? ; vars . push ((key , val)) ; start = i + 1 ; } } Ok (vars) }
}

macro_rules! get_raw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_raw in module {}", module_path!());
    };
}

mkfn!{
    get_raw_introspect!();
    unsafe fn get_raw (shell : NonNull < r_efi :: efi :: protocols :: shell :: Protocol > , key_ptr : * mut r_efi :: efi :: Char16 ,) -> Option < OsString > { let val = unsafe { ((* shell . as_ptr ()) . get_env) (key_ptr) } ; helpers :: os_string_from_raw (val) }
}

macro_rules! set_raw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_raw in module {}", module_path!());
    };
}

mkfn!{
    set_raw_introspect!();
    unsafe fn set_raw (key_ptr : * mut r_efi :: efi :: Char16 , val_ptr : * mut r_efi :: efi :: Char16 ,) -> io :: Result < () > { let shell = helpers :: open_shell () . ok_or (unsupported_err ()) ? ; let r = unsafe { ((* shell . as_ptr ()) . set_env) (key_ptr , val_ptr , r_efi :: efi :: Boolean :: FALSE) } ; if r . is_error () { Err (io :: Error :: from_raw_os_error (r . as_usize ())) } else { Ok (()) } }
} 
            }}