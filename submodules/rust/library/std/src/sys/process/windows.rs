mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkuse!{use core :: ffi :: c_void ;}
mkuse!{use super :: env :: { CommandEnv , CommandEnvs } ;}
mkuse!{use crate :: collections :: BTreeMap ;}
mkuse!{use crate :: env :: consts :: { EXE_EXTENSION , EXE_SUFFIX } ;}
mkuse!{use crate :: ffi :: { OsStr , OsString } ;}
mkuse!{use crate :: io :: { self , Error } ;}
mkuse!{use crate :: num :: NonZero ;}
mkuse!{use crate :: os :: windows :: ffi :: { OsStrExt , OsStringExt } ;}
mkuse!{use crate :: os :: windows :: io :: { AsHandle , AsRawHandle , BorrowedHandle , FromRawHandle , IntoRawHandle } ;}
mkuse!{use crate :: os :: windows :: process :: ProcThreadAttributeList ;}
mkuse!{use crate :: path :: { Path , PathBuf } ;}
mkuse!{use crate :: sync :: Mutex ;}
mkuse!{use crate :: sys :: args :: { self , Arg } ;}
mkuse!{use crate :: sys :: c :: { self , EXIT_FAILURE , EXIT_SUCCESS } ;}
mkuse!{use crate :: sys :: fs :: { File , OpenOptions } ;}
mkuse!{use crate :: sys :: handle :: Handle ;}
mkuse!{use crate :: sys :: pal :: api :: { self , WinError , utf16 } ;}
mkuse!{use crate :: sys :: pal :: { ensure_no_nuls , fill_utf16_buf } ;}
mkuse!{use crate :: sys :: pipe :: { self , AnonPipe } ;}
mkuse!{use crate :: sys :: { cvt , path , stdio } ;}
mkuse!{use crate :: sys_common :: IntoInner ;}
mkuse!{use crate :: { cmp , env , fmt , ptr } ;}
mkitem!{mkstruct!{# [derive (Clone , Debug , Eq)] # [doc (hidden)] pub struct EnvKey { os_string : OsString , utf16 : Vec < u16 > , }}}
mkitem!{mkimpl!{impl EnvKey { fn new < T : Into < OsString > > (key : T) -> Self { EnvKey :: from (key . into ()) } }}}
mkitem!{mkimpl!{impl Ord for EnvKey { fn cmp (& self , other : & Self) -> cmp :: Ordering { unsafe { let result = c :: CompareStringOrdinal (self . utf16 . as_ptr () , self . utf16 . len () as _ , other . utf16 . as_ptr () , other . utf16 . len () as _ , c :: TRUE ,) ; match result { c :: CSTR_LESS_THAN => cmp :: Ordering :: Less , c :: CSTR_EQUAL => cmp :: Ordering :: Equal , c :: CSTR_GREATER_THAN => cmp :: Ordering :: Greater , _ => panic ! ("comparing environment keys failed: {}" , Error :: last_os_error ()) , } } } }}}
mkitem!{mkimpl!{impl PartialOrd for EnvKey { fn partial_cmp (& self , other : & Self) -> Option < cmp :: Ordering > { Some (self . cmp (other)) } }}}
mkitem!{mkimpl!{impl PartialEq for EnvKey { fn eq (& self , other : & Self) -> bool { if self . utf16 . len () != other . utf16 . len () { false } else { self . cmp (other) == cmp :: Ordering :: Equal } } }}}
mkitem!{mkimpl!{impl PartialOrd < str > for EnvKey { fn partial_cmp (& self , other : & str) -> Option < cmp :: Ordering > { Some (self . cmp (& EnvKey :: new (other))) } }}}
mkitem!{mkimpl!{impl PartialEq < str > for EnvKey { fn eq (& self , other : & str) -> bool { if self . os_string . len () != other . len () { false } else { self . cmp (& EnvKey :: new (other)) == cmp :: Ordering :: Equal } } }}}
mkitem!{mkimpl!{impl From < OsString > for EnvKey { fn from (k : OsString) -> Self { EnvKey { utf16 : k . encode_wide () . collect () , os_string : k } } }}}
mkitem!{mkimpl!{impl From < EnvKey > for OsString { fn from (k : EnvKey) -> Self { k . os_string } }}}
mkitem!{mkimpl!{impl From < & OsStr > for EnvKey { fn from (k : & OsStr) -> Self { Self :: from (k . to_os_string ()) } }}}
mkitem!{mkimpl!{impl AsRef < OsStr > for EnvKey { fn as_ref (& self) -> & OsStr { & self . os_string } }}}
mkitem!{mkstruct!{pub struct Command { program : OsString , args : Vec < Arg > , env : CommandEnv , cwd : Option < OsString > , flags : u32 , show_window : Option < u16 > , detach : bool , stdin : Option < Stdio > , stdout : Option < Stdio > , stderr : Option < Stdio > , force_quotes_enabled : bool , startupinfo_fullscreen : bool , startupinfo_untrusted_source : bool , startupinfo_force_feedback : Option < bool > , }}}
mkitem!{mkenum!{pub enum Stdio { Inherit , InheritSpecific { from_stdio_id : u32 } , Null , MakePipe , Pipe (AnonPipe) , Handle (Handle) , }}}
mkitem!{mkstruct!{pub struct StdioPipes { pub stdin : Option < AnonPipe > , pub stdout : Option < AnonPipe > , pub stderr : Option < AnonPipe > , }}}
mkitem!{mkimpl!{impl Command { pub fn new (program : & OsStr) -> Command { Command { program : program . to_os_string () , args : Vec :: new () , env : Default :: default () , cwd : None , flags : 0 , show_window : None , detach : false , stdin : None , stdout : None , stderr : None , force_quotes_enabled : false , startupinfo_fullscreen : false , startupinfo_untrusted_source : false , startupinfo_force_feedback : None , } } pub fn arg (& mut self , arg : & OsStr) { self . args . push (Arg :: Regular (arg . to_os_string ())) } pub fn env_mut (& mut self) -> & mut CommandEnv { & mut self . env } pub fn cwd (& mut self , dir : & OsStr) { self . cwd = Some (dir . to_os_string ()) } pub fn stdin (& mut self , stdin : Stdio) { self . stdin = Some (stdin) ; } pub fn stdout (& mut self , stdout : Stdio) { self . stdout = Some (stdout) ; } pub fn stderr (& mut self , stderr : Stdio) { self . stderr = Some (stderr) ; } pub fn creation_flags (& mut self , flags : u32) { self . flags = flags ; } pub fn show_window (& mut self , cmd_show : Option < u16 >) { self . show_window = cmd_show ; } pub fn force_quotes (& mut self , enabled : bool) { self . force_quotes_enabled = enabled ; } pub fn raw_arg (& mut self , command_str_to_append : & OsStr) { self . args . push (Arg :: Raw (command_str_to_append . to_os_string ())) } pub fn startupinfo_fullscreen (& mut self , enabled : bool) { self . startupinfo_fullscreen = enabled ; } pub fn startupinfo_untrusted_source (& mut self , enabled : bool) { self . startupinfo_untrusted_source = enabled ; } pub fn startupinfo_force_feedback (& mut self , enabled : Option < bool >) { self . startupinfo_force_feedback = enabled ; } pub fn get_program (& self) -> & OsStr { & self . program } pub fn get_args (& self) -> CommandArgs < '_ > { let iter = self . args . iter () ; CommandArgs { iter } } pub fn get_envs (& self) -> CommandEnvs < '_ > { self . env . iter () } pub fn get_current_dir (& self) -> Option < & Path > { self . cwd . as_ref () . map (Path :: new) } pub fn spawn (& mut self , default : Stdio , needs_stdin : bool ,) -> io :: Result < (Process , StdioPipes) > { self . spawn_with_attributes (default , needs_stdin , None) } pub fn spawn_with_attributes (& mut self , default : Stdio , needs_stdin : bool , proc_thread_attribute_list : Option < & ProcThreadAttributeList < '_ > > ,) -> io :: Result < (Process , StdioPipes) > { let env_saw_path = self . env . have_changed_path () ; let maybe_env = self . env . capture_if_changed () ; let child_paths = if env_saw_path && let Some (env) = maybe_env . as_ref () { env . get (& EnvKey :: new ("PATH")) . map (| s | s . as_os_str ()) } else { None } ; let program = resolve_exe (& self . program , | | env :: var_os ("PATH") , child_paths) ? ; let has_bat_extension = | program : & [u16] | { matches ! (program . len () . checked_sub (4) . and_then (| i | program . get (i ..)) , Some ([46 , 98 | 66 , 97 | 65 , 116 | 84] | [46 , 99 | 67 , 109 | 77 , 100 | 68])) } ; let is_batch_file = if path :: is_verbatim (& program) { has_bat_extension (& program [.. program . len () - 1]) } else { fill_utf16_buf (| buffer , size | unsafe { c :: GetFullPathNameW (program . as_ptr () , size , buffer , ptr :: null_mut ()) } , | program | has_bat_extension (program) ,) ? } ; let (program , mut cmd_str) = if is_batch_file { (command_prompt () ? , args :: make_bat_command_line (& program , & self . args , self . force_quotes_enabled) ? ,) } else { let cmd_str = make_command_line (& self . program , & self . args , self . force_quotes_enabled) ? ; (program , cmd_str) } ; cmd_str . push (0) ; let mut flags = self . flags | c :: CREATE_UNICODE_ENVIRONMENT ; if self . detach { flags |= c :: DETACHED_PROCESS | c :: CREATE_NEW_PROCESS_GROUP ; } let (envp , _data) = make_envp (maybe_env) ? ; let (dirp , _data) = make_dirp (self . cwd . as_ref ()) ? ; let mut pi = zeroed_process_information () ; static CREATE_PROCESS_LOCK : Mutex < () > = Mutex :: new (()) ; let _guard = CREATE_PROCESS_LOCK . lock () ; let mut pipes = StdioPipes { stdin : None , stdout : None , stderr : None } ; let null = Stdio :: Null ; let default_stdin = if needs_stdin { & default } else { & null } ; let stdin = self . stdin . as_ref () . unwrap_or (default_stdin) ; let stdout = self . stdout . as_ref () . unwrap_or (& default) ; let stderr = self . stderr . as_ref () . unwrap_or (& default) ; let stdin = stdin . to_handle (c :: STD_INPUT_HANDLE , & mut pipes . stdin) ? ; let stdout = stdout . to_handle (c :: STD_OUTPUT_HANDLE , & mut pipes . stdout) ? ; let stderr = stderr . to_handle (c :: STD_ERROR_HANDLE , & mut pipes . stderr) ? ; let mut si = zeroed_startupinfo () ; let is_set = | stdio : & Handle | ! stdio . as_raw_handle () . is_null () ; if is_set (& stderr) || is_set (& stdout) || is_set (& stdin) { si . dwFlags |= c :: STARTF_USESTDHANDLES ; si . hStdInput = stdin . as_raw_handle () ; si . hStdOutput = stdout . as_raw_handle () ; si . hStdError = stderr . as_raw_handle () ; } if let Some (cmd_show) = self . show_window { si . dwFlags |= c :: STARTF_USESHOWWINDOW ; si . wShowWindow = cmd_show ; } if self . startupinfo_fullscreen { si . dwFlags |= c :: STARTF_RUNFULLSCREEN ; } if self . startupinfo_untrusted_source { si . dwFlags |= c :: STARTF_UNTRUSTEDSOURCE ; } match self . startupinfo_force_feedback { Some (true) => { si . dwFlags |= c :: STARTF_FORCEONFEEDBACK ; } Some (false) => { si . dwFlags |= c :: STARTF_FORCEOFFFEEDBACK ; } None => { } } let si_ptr : * mut c :: STARTUPINFOW ; let mut si_ex ; if let Some (proc_thread_attribute_list) = proc_thread_attribute_list { si . cb = size_of :: < c :: STARTUPINFOEXW > () as u32 ; flags |= c :: EXTENDED_STARTUPINFO_PRESENT ; si_ex = c :: STARTUPINFOEXW { StartupInfo : si , lpAttributeList : proc_thread_attribute_list . as_ptr () . cast :: < c_void > () . cast_mut () , } ; si_ptr = (& raw mut si_ex) as _ ; } else { si . cb = size_of :: < c :: STARTUPINFOW > () as u32 ; si_ptr = (& raw mut si) as _ ; } unsafe { cvt (c :: CreateProcessW (program . as_ptr () , cmd_str . as_mut_ptr () , ptr :: null_mut () , ptr :: null_mut () , c :: TRUE , flags , envp , dirp , si_ptr , & mut pi ,)) } ? ; unsafe { Ok ((Process { handle : Handle :: from_raw_handle (pi . hProcess) , main_thread_handle : Handle :: from_raw_handle (pi . hThread) , } , pipes ,)) } } }}}
mkitem!{mkimpl!{impl fmt :: Debug for Command { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . program . fmt (f) ? ; for arg in & self . args { f . write_str (" ") ? ; match arg { Arg :: Regular (s) => s . fmt (f) , Arg :: Raw (s) => f . write_str (& s . to_string_lossy ()) , } ? ; } Ok (()) } }}}

macro_rules! resolve_exe_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function resolve_exe in module {}", module_path!());
    };
}

mkfn!{
    resolve_exe_introspect!();
    fn resolve_exe < 'a > (exe_path : & 'a OsStr , parent_paths : impl FnOnce () -> Option < OsString > , child_paths : Option < & OsStr > ,) -> io :: Result < Vec < u16 > > { if exe_path . is_empty () || path :: has_trailing_slash (exe_path) { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "program path has no file name")) ; } let has_exe_suffix = if exe_path . len () >= EXE_SUFFIX . len () { exe_path . as_encoded_bytes () [exe_path . len () - EXE_SUFFIX . len () ..] . eq_ignore_ascii_case (EXE_SUFFIX . as_bytes ()) } else { false } ; if ! path :: is_file_name (exe_path) { if has_exe_suffix { return args :: to_user_path (Path :: new (exe_path)) ; } let mut path = PathBuf :: from (exe_path) ; path = path :: append_suffix (path , EXE_SUFFIX . as_ref ()) ; if let Some (path) = program_exists (& path) { return Ok (path) ; } else { path . set_extension ("") ; return args :: to_user_path (& path) ; } } else { ensure_no_nuls (exe_path) ? ; let has_extension = exe_path . as_encoded_bytes () . contains (& b'.') ; let result = search_paths (parent_paths , child_paths , | mut path | { path . push (exe_path) ; if ! has_extension { path . set_extension (EXE_EXTENSION) ; } program_exists (& path) }) ; if let Some (path) = result { return Ok (path) ; } } Err (io :: const_error ! (io :: ErrorKind :: NotFound , "program not found")) }
}

macro_rules! search_paths_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function search_paths in module {}", module_path!());
    };
}

mkfn!{
    search_paths_introspect!();
    fn search_paths < Paths , Exists > (parent_paths : Paths , child_paths : Option < & OsStr > , mut exists : Exists ,) -> Option < Vec < u16 > > where Paths : FnOnce () -> Option < OsString > , Exists : FnMut (PathBuf) -> Option < Vec < u16 > > , { if let Some (paths) = child_paths { for path in env :: split_paths (paths) . filter (| p | ! p . as_os_str () . is_empty ()) { if let Some (path) = exists (path) { return Some (path) ; } } } if let Ok (mut app_path) = env :: current_exe () { app_path . pop () ; if let Some (path) = exists (app_path) { return Some (path) ; } } unsafe { if let Ok (Some (path)) = fill_utf16_buf (| buf , size | c :: GetSystemDirectoryW (buf , size) , | buf | exists (PathBuf :: from (OsString :: from_wide (buf))) ,) { return Some (path) ; } # [cfg (not (target_vendor = "uwp"))] { if let Ok (Some (path)) = fill_utf16_buf (| buf , size | c :: GetWindowsDirectoryW (buf , size) , | buf | exists (PathBuf :: from (OsString :: from_wide (buf))) ,) { return Some (path) ; } } } if let Some (parent_paths) = parent_paths () { for path in env :: split_paths (& parent_paths) . filter (| p | ! p . as_os_str () . is_empty ()) { if let Some (path) = exists (path) { return Some (path) ; } } } None }
}

macro_rules! program_exists_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function program_exists in module {}", module_path!());
    };
}

mkfn!{
    program_exists_introspect!();
    # [doc = " Checks if a file exists without following symlinks."] fn program_exists (path : & Path) -> Option < Vec < u16 > > { unsafe { let path = args :: to_user_path (path) . ok () ? ; if c :: GetFileAttributesW (path . as_ptr ()) == c :: INVALID_FILE_ATTRIBUTES { None } else { Some (path) } } }
}
mkitem!{mkimpl!{impl Stdio { fn to_handle (& self , stdio_id : u32 , pipe : & mut Option < AnonPipe >) -> io :: Result < Handle > { let use_stdio_id = | stdio_id | match stdio :: get_handle (stdio_id) { Ok (io) => unsafe { let io = Handle :: from_raw_handle (io) ; let ret = io . duplicate (0 , true , c :: DUPLICATE_SAME_ACCESS) ; let _ = io . into_raw_handle () ; ret } , Err (..) => unsafe { Ok (Handle :: from_raw_handle (ptr :: null_mut ())) } , } ; match * self { Stdio :: Inherit => use_stdio_id (stdio_id) , Stdio :: InheritSpecific { from_stdio_id } => use_stdio_id (from_stdio_id) , Stdio :: MakePipe => { let ours_readable = stdio_id != c :: STD_INPUT_HANDLE ; let pipes = pipe :: anon_pipe (ours_readable , true) ? ; * pipe = Some (pipes . ours) ; Ok (pipes . theirs . into_handle ()) } Stdio :: Pipe (ref source) => { let ours_readable = stdio_id != c :: STD_INPUT_HANDLE ; pipe :: spawn_pipe_relay (source , ours_readable , true) . map (AnonPipe :: into_handle) } Stdio :: Handle (ref handle) => handle . duplicate (0 , true , c :: DUPLICATE_SAME_ACCESS) , Stdio :: Null => { let mut opts = OpenOptions :: new () ; opts . read (stdio_id == c :: STD_INPUT_HANDLE) ; opts . write (stdio_id != c :: STD_INPUT_HANDLE) ; opts . inherit_handle (true) ; File :: open (Path :: new (r"\\.\NUL") , & opts) . map (| file | file . into_inner ()) } } } }}}
mkitem!{mkimpl!{impl From < AnonPipe > for Stdio { fn from (pipe : AnonPipe) -> Stdio { Stdio :: Pipe (pipe) } }}}
mkitem!{mkimpl!{impl From < Handle > for Stdio { fn from (pipe : Handle) -> Stdio { Stdio :: Handle (pipe) } }}}
mkitem!{mkimpl!{impl From < File > for Stdio { fn from (file : File) -> Stdio { Stdio :: Handle (file . into_inner ()) } }}}
mkitem!{mkimpl!{impl From < io :: Stdout > for Stdio { fn from (_ : io :: Stdout) -> Stdio { Stdio :: InheritSpecific { from_stdio_id : c :: STD_OUTPUT_HANDLE } } }}}
mkitem!{mkimpl!{impl From < io :: Stderr > for Stdio { fn from (_ : io :: Stderr) -> Stdio { Stdio :: InheritSpecific { from_stdio_id : c :: STD_ERROR_HANDLE } } }}}
mkitem!{mkstruct!{# [doc = " A value representing a child process."] # [doc = ""] # [doc = " The lifetime of this value is linked to the lifetime of the actual"] # [doc = " process - the Process destructor calls self.finish() which waits"] # [doc = " for the process to terminate."] pub struct Process { handle : Handle , main_thread_handle : Handle , }}}
mkitem!{mkimpl!{impl Process { pub fn kill (& mut self) -> io :: Result < () > { let result = unsafe { c :: TerminateProcess (self . handle . as_raw_handle () , 1) } ; if result == c :: FALSE { let error = api :: get_last_error () ; if error != WinError :: ACCESS_DENIED || self . try_wait () . is_err () { return Err (crate :: io :: Error :: from_raw_os_error (error . code as i32)) ; } } Ok (()) } pub fn id (& self) -> u32 { unsafe { c :: GetProcessId (self . handle . as_raw_handle ()) } } pub fn main_thread_handle (& self) -> BorrowedHandle < '_ > { self . main_thread_handle . as_handle () } pub fn wait (& mut self) -> io :: Result < ExitStatus > { unsafe { let res = c :: WaitForSingleObject (self . handle . as_raw_handle () , c :: INFINITE) ; if res != c :: WAIT_OBJECT_0 { return Err (Error :: last_os_error ()) ; } let mut status = 0 ; cvt (c :: GetExitCodeProcess (self . handle . as_raw_handle () , & mut status)) ? ; Ok (ExitStatus (status)) } } pub fn try_wait (& mut self) -> io :: Result < Option < ExitStatus > > { unsafe { match c :: WaitForSingleObject (self . handle . as_raw_handle () , 0) { c :: WAIT_OBJECT_0 => { } c :: WAIT_TIMEOUT => { return Ok (None) ; } _ => return Err (io :: Error :: last_os_error ()) , } let mut status = 0 ; cvt (c :: GetExitCodeProcess (self . handle . as_raw_handle () , & mut status)) ? ; Ok (Some (ExitStatus (status))) } } pub fn handle (& self) -> & Handle { & self . handle } pub fn into_handle (self) -> Handle { self . handle } }}}
mkitem!{mkstruct!{# [derive (PartialEq , Eq , Clone , Copy , Debug , Default)] pub struct ExitStatus (u32) ;}}
mkitem!{mkimpl!{impl ExitStatus { pub fn exit_ok (& self) -> Result < () , ExitStatusError > { match NonZero :: < u32 > :: try_from (self . 0) { Ok (failure) => Err (ExitStatusError (failure)) , Err (_) => Ok (()) , } } pub fn code (& self) -> Option < i32 > { Some (self . 0 as i32) } }}}
mkitem!{mkimpl!{# [doc = " Converts a raw `u32` to a type-safe `ExitStatus` by wrapping it without copying."] impl From < u32 > for ExitStatus { fn from (u : u32) -> ExitStatus { ExitStatus (u) } }}}
mkitem!{mkimpl!{impl fmt :: Display for ExitStatus { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . 0 & 0x80000000 != 0 { write ! (f , "exit code: {:#x}" , self . 0) } else { write ! (f , "exit code: {}" , self . 0) } } }}}
mkitem!{mkstruct!{# [derive (PartialEq , Eq , Clone , Copy , Debug)] pub struct ExitStatusError (NonZero < u32 >) ;}}
mkitem!{mkimpl!{impl Into < ExitStatus > for ExitStatusError { fn into (self) -> ExitStatus { ExitStatus (self . 0 . into ()) } }}}
mkitem!{mkimpl!{impl ExitStatusError { pub fn code (self) -> Option < NonZero < i32 > > { Some ((u32 :: from (self . 0) as i32) . try_into () . unwrap ()) } }}}
mkitem!{mkstruct!{# [derive (PartialEq , Eq , Clone , Copy , Debug)] pub struct ExitCode (u32) ;}}
mkitem!{mkimpl!{impl ExitCode { pub const SUCCESS : ExitCode = ExitCode (EXIT_SUCCESS as _) ; pub const FAILURE : ExitCode = ExitCode (EXIT_FAILURE as _) ; # [inline] pub fn as_i32 (& self) -> i32 { self . 0 as i32 } }}}
mkitem!{mkimpl!{impl From < u8 > for ExitCode { fn from (code : u8) -> Self { ExitCode (u32 :: from (code)) } }}}
mkitem!{mkimpl!{impl From < u32 > for ExitCode { fn from (code : u32) -> Self { ExitCode (u32 :: from (code)) } }}}

macro_rules! zeroed_startupinfo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function zeroed_startupinfo in module {}", module_path!());
    };
}

mkfn!{
    zeroed_startupinfo_introspect!();
    fn zeroed_startupinfo () -> c :: STARTUPINFOW { c :: STARTUPINFOW { cb : 0 , lpReserved : ptr :: null_mut () , lpDesktop : ptr :: null_mut () , lpTitle : ptr :: null_mut () , dwX : 0 , dwY : 0 , dwXSize : 0 , dwYSize : 0 , dwXCountChars : 0 , dwYCountChars : 0 , dwFillAttribute : 0 , dwFlags : 0 , wShowWindow : 0 , cbReserved2 : 0 , lpReserved2 : ptr :: null_mut () , hStdInput : ptr :: null_mut () , hStdOutput : ptr :: null_mut () , hStdError : ptr :: null_mut () , } }
}

macro_rules! zeroed_process_information_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function zeroed_process_information in module {}", module_path!());
    };
}

mkfn!{
    zeroed_process_information_introspect!();
    fn zeroed_process_information () -> c :: PROCESS_INFORMATION { c :: PROCESS_INFORMATION { hProcess : ptr :: null_mut () , hThread : ptr :: null_mut () , dwProcessId : 0 , dwThreadId : 0 , } }
}

macro_rules! make_command_line_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_command_line in module {}", module_path!());
    };
}

mkfn!{
    make_command_line_introspect!();
    fn make_command_line (argv0 : & OsStr , args : & [Arg] , force_quotes : bool) -> io :: Result < Vec < u16 > > { let mut cmd : Vec < u16 > = Vec :: new () ; cmd . push (b'"' as u16) ; cmd . extend (argv0 . encode_wide ()) ; cmd . push (b'"' as u16) ; for arg in args { cmd . push (' ' as u16) ; args :: append_arg (& mut cmd , arg , force_quotes) ? ; } Ok (cmd) }
}

macro_rules! command_prompt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function command_prompt in module {}", module_path!());
    };
}

mkfn!{
    command_prompt_introspect!();
    fn command_prompt () -> io :: Result < Vec < u16 > > { let mut system : Vec < u16 > = fill_utf16_buf (| buf , size | unsafe { c :: GetSystemDirectoryW (buf , size) } , | buf | buf . into ()) ? ; system . extend ("\\cmd.exe" . encode_utf16 () . chain ([0])) ; Ok (system) }
}

macro_rules! make_envp_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_envp in module {}", module_path!());
    };
}

mkfn!{
    make_envp_introspect!();
    fn make_envp (maybe_env : Option < BTreeMap < EnvKey , OsString > >) -> io :: Result < (* mut c_void , Vec < u16 >) > { if let Some (env) = maybe_env { let mut blk = Vec :: new () ; if env . is_empty () { blk . push (0) ; } for (k , v) in env { ensure_no_nuls (k . os_string) ? ; blk . extend (k . utf16) ; blk . push ('=' as u16) ; blk . extend (ensure_no_nuls (v) ? . encode_wide ()) ; blk . push (0) ; } blk . push (0) ; Ok ((blk . as_mut_ptr () as * mut c_void , blk)) } else { Ok ((ptr :: null_mut () , Vec :: new ())) } }
}

macro_rules! make_dirp_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_dirp in module {}", module_path!());
    };
}

mkfn!{
    make_dirp_introspect!();
    fn make_dirp (d : Option < & OsString >) -> io :: Result < (* const u16 , Vec < u16 >) > { match d { Some (dir) => { let mut dir_str : Vec < u16 > = ensure_no_nuls (dir) ? . encode_wide () . chain ([0]) . collect () ; let ptr = if dir_str . starts_with (utf16 ! (r"\\?\UNC")) { let start = r"\\?\UN" . len () ; dir_str [start] = b'\\' as u16 ; if path :: is_absolute_exact (& dir_str [start ..]) { dir_str [start ..] . as_ptr () } else { dir_str [start] = b'C' as u16 ; dir_str . as_ptr () } } else if dir_str . starts_with (utf16 ! (r"\\?\")) { let start = r"\\?\" . len () ; if path :: is_absolute_exact (& dir_str [start ..]) { dir_str [start ..] . as_ptr () } else { dir_str . as_ptr () } } else { dir_str . as_ptr () } ; Ok ((ptr , dir_str)) } None => Ok ((ptr :: null () , Vec :: new ())) , } }
}
mkitem!{mkstruct!{pub struct CommandArgs < 'a > { iter : crate :: slice :: Iter < 'a , Arg > , }}}
mkitem!{mkimpl!{impl < 'a > Iterator for CommandArgs < 'a > { type Item = & 'a OsStr ; fn next (& mut self) -> Option < & 'a OsStr > { self . iter . next () . map (| arg | match arg { Arg :: Regular (s) | Arg :: Raw (s) => s . as_ref () , }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }}}
mkitem!{mkimpl!{impl < 'a > ExactSizeIterator for CommandArgs < 'a > { fn len (& self) -> usize { self . iter . len () } fn is_empty (& self) -> bool { self . iter . is_empty () } }}}
mkitem!{mkimpl!{impl < 'a > fmt :: Debug for CommandArgs < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter . clone ()) . finish () } }}}