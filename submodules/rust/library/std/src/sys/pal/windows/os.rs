mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkuse!{use super :: api ;}
mkuse!{# [cfg (not (target_vendor = "uwp"))] use super :: api :: WinError ;}
mkuse!{use crate :: ffi :: { OsStr , OsString } ;}
mkuse!{use crate :: os :: windows :: ffi :: EncodeWide ;}
mkuse!{use crate :: os :: windows :: prelude :: * ;}
mkuse!{use crate :: path :: { self , PathBuf } ;}
mkuse!{use crate :: sys :: pal :: { c , cvt } ;}
mkuse!{use crate :: { fmt , io , ptr } ;}

macro_rules! errno_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function errno in module {}", module_path!());
    };
}

mkfn!{
    errno_introspect!();
    pub fn errno () -> i32 { api :: get_last_error () . code as i32 }
}

macro_rules! error_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function error_string in module {}", module_path!());
    };
}

mkfn!{
    error_string_introspect!();
    # [doc = " Gets a detailed string description for the given error number."] pub fn error_string (mut errnum : i32) -> String { let mut buf = [0 as c :: WCHAR ; 2048] ; unsafe { let mut module = ptr :: null_mut () ; let mut flags = 0 ; if (errnum & c :: FACILITY_NT_BIT as i32) != 0 { const NTDLL_DLL : & [u16] = & ['N' as _ , 'T' as _ , 'D' as _ , 'L' as _ , 'L' as _ , '.' as _ , 'D' as _ , 'L' as _ , 'L' as _ , 0 ,] ; module = c :: GetModuleHandleW (NTDLL_DLL . as_ptr ()) ; if ! module . is_null () { errnum ^= c :: FACILITY_NT_BIT as i32 ; flags = c :: FORMAT_MESSAGE_FROM_HMODULE ; } } let res = c :: FormatMessageW (flags | c :: FORMAT_MESSAGE_FROM_SYSTEM | c :: FORMAT_MESSAGE_IGNORE_INSERTS , module , errnum as u32 , 0 , buf . as_mut_ptr () , buf . len () as u32 , ptr :: null () ,) as usize ; if res == 0 { let fm_err = errno () ; return format ! ("OS Error {errnum} (FormatMessageW() returned error {fm_err})") ; } match String :: from_utf16 (& buf [.. res]) { Ok (mut msg) => { let len = msg . trim_end () . len () ; msg . truncate (len) ; msg } Err (..) => format ! ("OS Error {} (FormatMessageW() returned \
                 invalid UTF-16)" , errnum) , } } }
}
mkitem!{mkstruct!{pub struct SplitPaths < 'a > { data : EncodeWide < 'a > , must_yield : bool , }}}

macro_rules! split_paths_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function split_paths in module {}", module_path!());
    };
}

mkfn!{
    split_paths_introspect!();
    pub fn split_paths (unparsed : & OsStr) -> SplitPaths < '_ > { SplitPaths { data : unparsed . encode_wide () , must_yield : true } }
}
mkitem!{mkimpl!{impl < 'a > Iterator for SplitPaths < 'a > { type Item = PathBuf ; fn next (& mut self) -> Option < PathBuf > { let must_yield = self . must_yield ; self . must_yield = false ; let mut in_progress = Vec :: new () ; let mut in_quote = false ; for b in self . data . by_ref () { if b == '"' as u16 { in_quote = ! in_quote ; } else if b == ';' as u16 && ! in_quote { self . must_yield = true ; break ; } else { in_progress . push (b) } } if ! must_yield && in_progress . is_empty () { None } else { Some (super :: os2path (& in_progress)) } } }}}
mkitem!{mkstruct!{# [derive (Debug)] pub struct JoinPathsError ;}}

macro_rules! join_paths_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function join_paths in module {}", module_path!());
    };
}

mkfn!{
    join_paths_introspect!();
    pub fn join_paths < I , T > (paths : I) -> Result < OsString , JoinPathsError > where I : Iterator < Item = T > , T : AsRef < OsStr > , { let mut joined = Vec :: new () ; let sep = b';' as u16 ; for (i , path) in paths . enumerate () { let path = path . as_ref () ; if i > 0 { joined . push (sep) } let v = path . encode_wide () . collect :: < Vec < u16 > > () ; if v . contains (& (b'"' as u16)) { return Err (JoinPathsError) ; } else if v . contains (& sep) { joined . push (b'"' as u16) ; joined . extend_from_slice (& v [..]) ; joined . push (b'"' as u16) ; } else { joined . extend_from_slice (& v [..]) ; } } Ok (OsStringExt :: from_wide (& joined [..])) }
}
mkitem!{mkimpl!{impl fmt :: Display for JoinPathsError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { "path segment contains `\"`" . fmt (f) } }}}
mkitem!{mkimpl!{impl crate :: error :: Error for JoinPathsError { }}}

macro_rules! current_exe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_exe in module {}", module_path!());
    };
}

mkfn!{
    current_exe_introspect!();
    pub fn current_exe () -> io :: Result < PathBuf > { super :: fill_utf16_buf (| buf , sz | unsafe { c :: GetModuleFileNameW (ptr :: null_mut () , buf , sz) } , super :: os2path ,) }
}

macro_rules! getcwd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getcwd in module {}", module_path!());
    };
}

mkfn!{
    getcwd_introspect!();
    pub fn getcwd () -> io :: Result < PathBuf > { super :: fill_utf16_buf (| buf , sz | unsafe { c :: GetCurrentDirectoryW (sz , buf) } , super :: os2path) }
}

macro_rules! chdir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function chdir in module {}", module_path!());
    };
}

mkfn!{
    chdir_introspect!();
    pub fn chdir (p : & path :: Path) -> io :: Result < () > { let p : & OsStr = p . as_ref () ; let mut p = p . encode_wide () . collect :: < Vec < _ > > () ; p . push (0) ; cvt (unsafe { c :: SetCurrentDirectoryW (p . as_ptr ()) }) . map (drop) }
}

macro_rules! temp_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function temp_dir in module {}", module_path!());
    };
}

mkfn!{
    temp_dir_introspect!();
    pub fn temp_dir () -> PathBuf { super :: fill_utf16_buf (| buf , sz | unsafe { c :: GetTempPath2W (sz , buf) } , super :: os2path) . unwrap () }
}

macro_rules! home_dir_crt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function home_dir_crt in module {}", module_path!());
    };
}

mkfn!{
    home_dir_crt_introspect!();
    # [cfg (all (not (target_vendor = "uwp") , not (target_vendor = "win7")))] fn home_dir_crt () -> Option < PathBuf > { unsafe { const CURRENT_PROCESS_TOKEN : usize = - 4_isize as usize ; super :: fill_utf16_buf (| buf , mut sz | { match c :: GetUserProfileDirectoryW (ptr :: without_provenance_mut (CURRENT_PROCESS_TOKEN) , buf , & mut sz ,) { 0 if api :: get_last_error () != WinError :: INSUFFICIENT_BUFFER => 0 , 0 => sz , _ => sz - 1 , } } , super :: os2path ,) . ok () } }
}

macro_rules! home_dir_crt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function home_dir_crt in module {}", module_path!());
    };
}

mkfn!{
    home_dir_crt_introspect!();
    # [cfg (target_vendor = "win7")] fn home_dir_crt () -> Option < PathBuf > { unsafe { use crate :: sys :: handle :: Handle ; let me = c :: GetCurrentProcess () ; let mut token = ptr :: null_mut () ; if c :: OpenProcessToken (me , c :: TOKEN_READ , & mut token) == 0 { return None ; } let _handle = Handle :: from_raw_handle (token) ; super :: fill_utf16_buf (| buf , mut sz | { match c :: GetUserProfileDirectoryW (token , buf , & mut sz) { 0 if api :: get_last_error () != WinError :: INSUFFICIENT_BUFFER => 0 , 0 => sz , _ => sz - 1 , } } , super :: os2path ,) . ok () } }
}

macro_rules! home_dir_crt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function home_dir_crt in module {}", module_path!());
    };
}

mkfn!{
    home_dir_crt_introspect!();
    # [cfg (target_vendor = "uwp")] fn home_dir_crt () -> Option < PathBuf > { None }
}

macro_rules! home_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function home_dir in module {}", module_path!());
    };
}

mkfn!{
    home_dir_introspect!();
    pub fn home_dir () -> Option < PathBuf > { crate :: env :: var_os ("USERPROFILE") . filter (| s | ! s . is_empty ()) . map (PathBuf :: from) . or_else (home_dir_crt) }
}

macro_rules! exit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exit in module {}", module_path!());
    };
}

mkfn!{
    exit_introspect!();
    pub fn exit (code : i32) -> ! { unsafe { c :: ExitProcess (code as u32) } }
}

macro_rules! getpid_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getpid in module {}", module_path!());
    };
}

mkfn!{
    getpid_introspect!();
    pub fn getpid () -> u32 { unsafe { c :: GetCurrentProcessId () } }
}