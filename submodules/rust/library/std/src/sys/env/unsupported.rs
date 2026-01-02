mkuse!{use crate :: ffi :: { OsStr , OsString } ;}
mkuse!{use crate :: { fmt , io } ;}
mkitem!{mkstruct!{pub struct Env (!) ;}}
mkitem!{mkimpl!{impl Env { pub fn str_debug (& self) -> impl fmt :: Debug + '_ { self . 0 } }}}
mkitem!{mkimpl!{impl fmt :: Debug for Env { fn fmt (& self , _ : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 } }}}
mkitem!{mkimpl!{impl Iterator for Env { type Item = (OsString , OsString) ; fn next (& mut self) -> Option < (OsString , OsString) > { self . 0 } }}}

macro_rules! env_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function env in module {}", module_path!());
    };
}

mkfn!{
    env_introspect!();
    pub fn env () -> Env { panic ! ("not supported on this platform") }
}

macro_rules! getenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getenv in module {}", module_path!());
    };
}

mkfn!{
    getenv_introspect!();
    pub fn getenv (_ : & OsStr) -> Option < OsString > { None }
}

macro_rules! setenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function setenv in module {}", module_path!());
    };
}

mkfn!{
    setenv_introspect!();
    pub unsafe fn setenv (_ : & OsStr , _ : & OsStr) -> io :: Result < () > { Err (io :: const_error ! (io :: ErrorKind :: Unsupported , "cannot set env vars on this platform")) }
}

macro_rules! unsetenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unsetenv in module {}", module_path!());
    };
}

mkfn!{
    unsetenv_introspect!();
    pub unsafe fn unsetenv (_ : & OsStr) -> io :: Result < () > { Err (io :: const_error ! (io :: ErrorKind :: Unsupported , "cannot unset env vars on this platform")) }
}