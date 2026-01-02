mkuse!{use core :: marker :: PhantomData ;}
mkuse!{use super :: unsupported ;}
mkuse!{use crate :: ffi :: { OsStr , OsString } ;}
mkuse!{use crate :: path :: PathBuf ;}
mkuse!{use crate :: { fmt , io , path } ;}

macro_rules! errno_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function errno in module {}", module_path!());
    };
}

mkfn!{
    errno_introspect!();
    pub fn errno () -> i32 { unsafe { (* libc :: __errno_location ()) as i32 } }
}

macro_rules! page_size_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function page_size in module {}", module_path!());
    };
}

mkfn!{
    page_size_introspect!();
    pub fn page_size () -> usize { 4096 }
}

macro_rules! error_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function error_string in module {}", module_path!());
    };
}

mkfn!{
    error_string_introspect!();
    pub fn error_string (_errno : i32) -> String { "error string unimplemented" . to_string () }
}

macro_rules! getcwd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getcwd in module {}", module_path!());
    };
}

mkfn!{
    getcwd_introspect!();
    pub fn getcwd () -> io :: Result < PathBuf > { unsupported () }
}

macro_rules! chdir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function chdir in module {}", module_path!());
    };
}

mkfn!{
    chdir_introspect!();
    pub fn chdir (_ : & path :: Path) -> io :: Result < () > { unsupported () }
}
mkitem!{mkstruct!{pub struct SplitPaths < 'a > (! , PhantomData < & 'a () >) ;}}

macro_rules! split_paths_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function split_paths in module {}", module_path!());
    };
}

mkfn!{
    split_paths_introspect!();
    pub fn split_paths (_unparsed : & OsStr) -> SplitPaths < '_ > { panic ! ("unsupported") }
}
mkitem!{mkimpl!{impl < 'a > Iterator for SplitPaths < 'a > { type Item = PathBuf ; fn next (& mut self) -> Option < PathBuf > { self . 0 } }}}
mkitem!{mkstruct!{# [derive (Debug)] pub struct JoinPathsError ;}}

macro_rules! join_paths_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function join_paths in module {}", module_path!());
    };
}

mkfn!{
    join_paths_introspect!();
    pub fn join_paths < I , T > (_paths : I) -> Result < OsString , JoinPathsError > where I : Iterator < Item = T > , T : AsRef < OsStr > , { Err (JoinPathsError) }
}
mkitem!{mkimpl!{impl fmt :: Display for JoinPathsError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { "not supported on this platform yet" . fmt (f) } }}}
mkitem!{mkimpl!{impl crate :: error :: Error for JoinPathsError { }}}

macro_rules! current_exe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_exe in module {}", module_path!());
    };
}

mkfn!{
    current_exe_introspect!();
    pub fn current_exe () -> io :: Result < PathBuf > { unsupported () }
}

macro_rules! temp_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function temp_dir in module {}", module_path!());
    };
}

mkfn!{
    temp_dir_introspect!();
    pub fn temp_dir () -> PathBuf { panic ! ("no filesystem on this platform") }
}

macro_rules! home_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function home_dir in module {}", module_path!());
    };
}

mkfn!{
    home_dir_introspect!();
    pub fn home_dir () -> Option < PathBuf > { None }
}

macro_rules! exit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exit in module {}", module_path!());
    };
}

mkfn!{
    exit_introspect!();
    pub fn exit (_code : i32) -> ! { panic ! ("TA should not call `exit`") }
}

macro_rules! getpid_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getpid in module {}", module_path!());
    };
}

mkfn!{
    getpid_introspect!();
    pub fn getpid () -> u32 { panic ! ("no pids on this platform") }
}