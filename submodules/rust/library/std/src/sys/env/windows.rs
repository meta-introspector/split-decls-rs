mkuse!{use crate :: ffi :: { OsStr , OsString } ;}
mkuse!{use crate :: os :: windows :: prelude :: * ;}
mkuse!{use crate :: sys :: pal :: { c , cvt , fill_utf16_buf , to_u16s } ;}
mkuse!{use crate :: { fmt , io , ptr , slice } ;}
mkitem!{mkstruct!{pub struct Env { base : * mut c :: WCHAR , iter : EnvIterator , }}}
mkitem!{mkstruct!{pub struct EnvStrDebug < 'a > { iter : & 'a EnvIterator , }}}
mkitem!{mkimpl!{impl fmt :: Debug for EnvStrDebug < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let Self { iter } = self ; let iter : EnvIterator = (* iter) . clone () ; let mut list = f . debug_list () ; for (a , b) in iter { list . entry (& (a . to_str () . unwrap () , b . to_str () . unwrap ())) ; } list . finish () } }}}
mkitem!{mkimpl!{impl Env { pub fn str_debug (& self) -> impl fmt :: Debug + '_ { let Self { base : _ , iter } = self ; EnvStrDebug { iter } } }}}
mkitem!{mkimpl!{impl fmt :: Debug for Env { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let Self { base : _ , iter } = self ; f . debug_list () . entries (iter . clone ()) . finish () } }}}
mkitem!{mkimpl!{impl Iterator for Env { type Item = (OsString , OsString) ; fn next (& mut self) -> Option < (OsString , OsString) > { let Self { base : _ , iter } = self ; iter . next () } }}}
mkitem!{mkstruct!{# [derive (Clone)] struct EnvIterator (* mut c :: WCHAR) ;}}
mkitem!{mkimpl!{impl Iterator for EnvIterator { type Item = (OsString , OsString) ; fn next (& mut self) -> Option < (OsString , OsString) > { let Self (cur) = self ; loop { unsafe { if * * cur == 0 { return None ; } let p = * cur as * const u16 ; let mut len = 0 ; while * p . add (len) != 0 { len += 1 ; } let s = slice :: from_raw_parts (p , len) ; * cur = cur . add (len + 1) ; let pos = match s [1 ..] . iter () . position (| & u | u == b'=' as u16) . map (| p | p + 1) { Some (p) => p , None => continue , } ; return Some ((OsStringExt :: from_wide (& s [.. pos]) , OsStringExt :: from_wide (& s [pos + 1 ..]) ,)) ; } } } }}}
mkitem!{mkimpl!{impl Drop for Env { fn drop (& mut self) { unsafe { c :: FreeEnvironmentStringsW (self . base) ; } } }}}

macro_rules! env_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function env in module {}", module_path!());
    };
}

mkfn!{
    env_introspect!();
    pub fn env () -> Env { unsafe { let ch = c :: GetEnvironmentStringsW () ; if ch . is_null () { panic ! ("failure getting env string from OS: {}" , io :: Error :: last_os_error ()) ; } Env { base : ch , iter : EnvIterator (ch) } } }
}

macro_rules! getenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getenv in module {}", module_path!());
    };
}

mkfn!{
    getenv_introspect!();
    pub fn getenv (k : & OsStr) -> Option < OsString > { let k = to_u16s (k) . ok () ? ; fill_utf16_buf (| buf , sz | unsafe { c :: GetEnvironmentVariableW (k . as_ptr () , buf , sz) } , OsStringExt :: from_wide ,) . ok () }
}

macro_rules! setenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function setenv in module {}", module_path!());
    };
}

mkfn!{
    setenv_introspect!();
    pub unsafe fn setenv (k : & OsStr , v : & OsStr) -> io :: Result < () > { unsafe { let k = to_u16s (k) ? ; let v = to_u16s (v) ? ; cvt (c :: SetEnvironmentVariableW (k . as_ptr () , v . as_ptr ())) . map (drop) } }
}

macro_rules! unsetenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unsetenv in module {}", module_path!());
    };
}

mkfn!{
    unsetenv_introspect!();
    pub unsafe fn unsetenv (n : & OsStr) -> io :: Result < () > { unsafe { let v = to_u16s (n) ? ; cvt (c :: SetEnvironmentVariableW (v . as_ptr () , ptr :: null ())) . map (drop) } }
}