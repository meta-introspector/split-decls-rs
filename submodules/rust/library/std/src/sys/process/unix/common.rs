mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkuse!{use libc :: { EXIT_FAILURE , EXIT_SUCCESS , c_int , gid_t , pid_t , uid_t } ;}
mkuse!{pub use self :: cstring_array :: CStringArray ;}
mkuse!{use self :: cstring_array :: CStringIter ;}
mkuse!{use crate :: collections :: BTreeMap ;}
mkuse!{use crate :: ffi :: { CStr , CString , OsStr , OsString } ;}
mkuse!{use crate :: os :: unix :: prelude :: * ;}
mkuse!{use crate :: path :: Path ;}
mkuse!{use crate :: sys :: fd :: FileDesc ;}
mkuse!{use crate :: sys :: fs :: File ;}
mkuse!{# [cfg (not (target_os = "fuchsia"))] use crate :: sys :: fs :: OpenOptions ;}
mkuse!{use crate :: sys :: pipe :: { self , AnonPipe } ;}
mkuse!{use crate :: sys :: process :: env :: { CommandEnv , CommandEnvs } ;}
mkuse!{use crate :: sys_common :: { FromInner , IntoInner } ;}
mkuse!{use crate :: { fmt , io } ;}
mkmod!{cstring_array, { 
                getname!(cstring_array);
                getsrc!(cstring_array);
                getpath!(cstring_array);
                get_deps!(cstring_array);
                get_crates!(cstring_array);
                mkinclude!(cstring_array);
                 
            }}
mkitem!{cfg_select ! { target_os = "fuchsia" => { } target_os = "vxworks" => { const DEV_NULL : & CStr = c"/null" ; } _ => { const DEV_NULL : & CStr = c"/dev/null" ; } }}
mkitem!{cfg_select ! { target_os = "android" => { # [allow (dead_code)] pub unsafe fn sigemptyset (set : * mut libc :: sigset_t) -> libc :: c_int { set . write_bytes (0u8 , 1) ; return 0 ; } # [allow (dead_code)] pub unsafe fn sigaddset (set : * mut libc :: sigset_t , signum : libc :: c_int) -> libc :: c_int { use crate :: slice ; use libc :: { c_ulong , sigset_t } ; const _ : () = assert ! (align_of ::< c_ulong > () == align_of ::< sigset_t > () && (size_of ::< sigset_t > () % size_of ::< c_ulong > ()) == 0) ; let bit = (signum - 1) as usize ; if set . is_null () || bit >= (8 * size_of ::< sigset_t > ()) { crate :: sys :: pal :: os :: set_errno (libc :: EINVAL) ; return - 1 ; } let raw = slice :: from_raw_parts_mut (set as * mut c_ulong , size_of ::< sigset_t > () / size_of ::< c_ulong > () ,) ; const LONG_BIT : usize = size_of ::< c_ulong > () * 8 ; raw [bit / LONG_BIT] |= 1 << (bit % LONG_BIT) ; return 0 ; } } _ => { # [allow (unused_imports)] pub use libc :: { sigemptyset , sigaddset } ; } }}
mkitem!{mkstruct!{pub struct Command { program : CString , args : CStringArray , env : CommandEnv , program_kind : ProgramKind , cwd : Option < CString > , chroot : Option < CString > , uid : Option < uid_t > , gid : Option < gid_t > , saw_nul : bool , closures : Vec < Box < dyn FnMut () -> io :: Result < () > + Send + Sync > > , groups : Option < Box < [gid_t] > > , stdin : Option < Stdio > , stdout : Option < Stdio > , stderr : Option < Stdio > , # [cfg (target_os = "linux")] create_pidfd : bool , pgroup : Option < pid_t > , setsid : bool , }}}
mkitem!{mkstruct!{pub struct StdioPipes { pub stdin : Option < AnonPipe > , pub stdout : Option < AnonPipe > , pub stderr : Option < AnonPipe > , }}}
mkitem!{mkstruct!{# [cfg_attr (target_os = "vita" , allow (dead_code))] pub struct ChildPipes { pub stdin : ChildStdio , pub stdout : ChildStdio , pub stderr : ChildStdio , }}}
mkitem!{mkenum!{pub enum ChildStdio { Inherit , Explicit (c_int) , Owned (FileDesc) , # [cfg (target_os = "fuchsia")] Null , }}}
mkitem!{mkenum!{# [derive (Debug)] pub enum Stdio { Inherit , Null , MakePipe , Fd (FileDesc) , StaticFd (BorrowedFd < 'static >) , }}}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum ProgramKind { # [doc = " A program that would be looked up on the PATH (e.g. `ls`)"] PathLookup , # [doc = " A relative path (e.g. `my-dir/foo`, `../foo`, `./foo`)"] Relative , # [doc = " An absolute path."] Absolute , }}}
mkitem!{mkimpl!{impl ProgramKind { fn new (program : & OsStr) -> Self { if program . as_encoded_bytes () . starts_with (b"/") { Self :: Absolute } else if program . as_encoded_bytes () . contains (& b'/') { Self :: Relative } else { Self :: PathLookup } } }}}
mkitem!{mkimpl!{impl Command { pub fn new (program : & OsStr) -> Command { let mut saw_nul = false ; let program_kind = ProgramKind :: new (program . as_ref ()) ; let program = os2c (program , & mut saw_nul) ; let mut args = CStringArray :: with_capacity (1) ; args . push (program . clone ()) ; Command { program , args , env : Default :: default () , program_kind , cwd : None , chroot : None , uid : None , gid : None , saw_nul , closures : Vec :: new () , groups : None , stdin : None , stdout : None , stderr : None , # [cfg (target_os = "linux")] create_pidfd : false , pgroup : None , setsid : false , } } pub fn set_arg_0 (& mut self , arg : & OsStr) { let arg = os2c (arg , & mut self . saw_nul) ; self . args . write (0 , arg) ; } pub fn arg (& mut self , arg : & OsStr) { let arg = os2c (arg , & mut self . saw_nul) ; self . args . push (arg) ; } pub fn cwd (& mut self , dir : & OsStr) { self . cwd = Some (os2c (dir , & mut self . saw_nul)) ; } pub fn uid (& mut self , id : uid_t) { self . uid = Some (id) ; } pub fn gid (& mut self , id : gid_t) { self . gid = Some (id) ; } pub fn groups (& mut self , groups : & [gid_t]) { self . groups = Some (Box :: from (groups)) ; } pub fn pgroup (& mut self , pgroup : pid_t) { self . pgroup = Some (pgroup) ; } pub fn chroot (& mut self , dir : & Path) { self . chroot = Some (os2c (dir . as_os_str () , & mut self . saw_nul)) ; if self . cwd . is_none () { self . cwd (& OsStr :: new ("/")) ; } } pub fn setsid (& mut self , setsid : bool) { self . setsid = setsid ; } # [cfg (target_os = "linux")] pub fn create_pidfd (& mut self , val : bool) { self . create_pidfd = val ; } # [cfg (not (target_os = "linux"))] # [allow (dead_code)] pub fn get_create_pidfd (& self) -> bool { false } # [cfg (target_os = "linux")] pub fn get_create_pidfd (& self) -> bool { self . create_pidfd } pub fn saw_nul (& self) -> bool { self . saw_nul } pub fn get_program (& self) -> & OsStr { OsStr :: from_bytes (self . program . as_bytes ()) } # [allow (dead_code)] pub fn get_program_kind (& self) -> ProgramKind { self . program_kind } pub fn get_args (& self) -> CommandArgs < '_ > { let mut iter = self . args . iter () ; iter . next () ; CommandArgs { iter } } pub fn get_envs (& self) -> CommandEnvs < '_ > { self . env . iter () } pub fn get_current_dir (& self) -> Option < & Path > { self . cwd . as_ref () . map (| cs | Path :: new (OsStr :: from_bytes (cs . as_bytes ()))) } pub fn get_argv (& self) -> & CStringArray { & self . args } pub fn get_program_cstr (& self) -> & CStr { & self . program } # [allow (dead_code)] pub fn get_cwd (& self) -> Option < & CStr > { self . cwd . as_deref () } # [allow (dead_code)] pub fn get_uid (& self) -> Option < uid_t > { self . uid } # [allow (dead_code)] pub fn get_gid (& self) -> Option < gid_t > { self . gid } # [allow (dead_code)] pub fn get_groups (& self) -> Option < & [gid_t] > { self . groups . as_deref () } # [allow (dead_code)] pub fn get_pgroup (& self) -> Option < pid_t > { self . pgroup } # [allow (dead_code)] pub fn get_chroot (& self) -> Option < & CStr > { self . chroot . as_deref () } # [allow (dead_code)] pub fn get_setsid (& self) -> bool { self . setsid } pub fn get_closures (& mut self) -> & mut Vec < Box < dyn FnMut () -> io :: Result < () > + Send + Sync > > { & mut self . closures } pub unsafe fn pre_exec (& mut self , f : Box < dyn FnMut () -> io :: Result < () > + Send + Sync >) { self . closures . push (f) ; } pub fn stdin (& mut self , stdin : Stdio) { self . stdin = Some (stdin) ; } pub fn stdout (& mut self , stdout : Stdio) { self . stdout = Some (stdout) ; } pub fn stderr (& mut self , stderr : Stdio) { self . stderr = Some (stderr) ; } pub fn env_mut (& mut self) -> & mut CommandEnv { & mut self . env } pub fn capture_env (& mut self) -> Option < CStringArray > { let maybe_env = self . env . capture_if_changed () ; maybe_env . map (| env | construct_envp (env , & mut self . saw_nul)) } # [allow (dead_code)] pub fn env_saw_path (& self) -> bool { self . env . have_changed_path () } # [allow (dead_code)] pub fn program_is_path (& self) -> bool { self . program . to_bytes () . contains (& b'/') } pub fn setup_io (& self , default : Stdio , needs_stdin : bool ,) -> io :: Result < (StdioPipes , ChildPipes) > { let null = Stdio :: Null ; let default_stdin = if needs_stdin { & default } else { & null } ; let stdin = self . stdin . as_ref () . unwrap_or (default_stdin) ; let stdout = self . stdout . as_ref () . unwrap_or (& default) ; let stderr = self . stderr . as_ref () . unwrap_or (& default) ; let (their_stdin , our_stdin) = stdin . to_child_stdio (true) ? ; let (their_stdout , our_stdout) = stdout . to_child_stdio (false) ? ; let (their_stderr , our_stderr) = stderr . to_child_stdio (false) ? ; let ours = StdioPipes { stdin : our_stdin , stdout : our_stdout , stderr : our_stderr } ; let theirs = ChildPipes { stdin : their_stdin , stdout : their_stdout , stderr : their_stderr } ; Ok ((ours , theirs)) } }}}

macro_rules! os2c_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function os2c in module {}", module_path!());
    };
}

mkfn!{
    os2c_introspect!();
    fn os2c (s : & OsStr , saw_nul : & mut bool) -> CString { CString :: new (s . as_bytes ()) . unwrap_or_else (| _e | { * saw_nul = true ; c"<string-with-nul>" . to_owned () }) }
}

macro_rules! construct_envp_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function construct_envp in module {}", module_path!());
    };
}

mkfn!{
    construct_envp_introspect!();
    fn construct_envp (env : BTreeMap < OsString , OsString > , saw_nul : & mut bool) -> CStringArray { let mut result = CStringArray :: with_capacity (env . len ()) ; for (mut k , v) in env { k . reserve_exact (v . len () + 2) ; k . push ("=") ; k . push (& v) ; if let Ok (item) = CString :: new (k . into_vec ()) { result . push (item) ; } else { * saw_nul = true ; } } result }
}
mkitem!{mkimpl!{impl Stdio { pub fn to_child_stdio (& self , readable : bool) -> io :: Result < (ChildStdio , Option < AnonPipe >) > { match * self { Stdio :: Inherit => Ok ((ChildStdio :: Inherit , None)) , Stdio :: Fd (ref fd) => { if fd . as_raw_fd () >= 0 && fd . as_raw_fd () <= libc :: STDERR_FILENO { Ok ((ChildStdio :: Owned (fd . duplicate () ?) , None)) } else { Ok ((ChildStdio :: Explicit (fd . as_raw_fd ()) , None)) } } Stdio :: StaticFd (fd) => { let fd = FileDesc :: from_inner (fd . try_clone_to_owned () ?) ; Ok ((ChildStdio :: Owned (fd) , None)) } Stdio :: MakePipe => { let (reader , writer) = pipe :: anon_pipe () ? ; let (ours , theirs) = if readable { (writer , reader) } else { (reader , writer) } ; Ok ((ChildStdio :: Owned (theirs . into_inner ()) , Some (ours))) } # [cfg (not (target_os = "fuchsia"))] Stdio :: Null => { let mut opts = OpenOptions :: new () ; opts . read (readable) ; opts . write (! readable) ; let fd = File :: open_c (DEV_NULL , & opts) ? ; Ok ((ChildStdio :: Owned (fd . into_inner ()) , None)) } # [cfg (target_os = "fuchsia")] Stdio :: Null => Ok ((ChildStdio :: Null , None)) , } } }}}
mkitem!{mkimpl!{impl From < AnonPipe > for Stdio { fn from (pipe : AnonPipe) -> Stdio { Stdio :: Fd (pipe . into_inner ()) } }}}
mkitem!{mkimpl!{impl From < FileDesc > for Stdio { fn from (fd : FileDesc) -> Stdio { Stdio :: Fd (fd) } }}}
mkitem!{mkimpl!{impl From < File > for Stdio { fn from (file : File) -> Stdio { Stdio :: Fd (file . into_inner ()) } }}}
mkitem!{mkimpl!{impl From < io :: Stdout > for Stdio { fn from (_ : io :: Stdout) -> Stdio { Stdio :: StaticFd (unsafe { BorrowedFd :: borrow_raw (libc :: STDOUT_FILENO) }) } }}}
mkitem!{mkimpl!{impl From < io :: Stderr > for Stdio { fn from (_ : io :: Stderr) -> Stdio { Stdio :: StaticFd (unsafe { BorrowedFd :: borrow_raw (libc :: STDERR_FILENO) }) } }}}
mkitem!{mkimpl!{impl ChildStdio { pub fn fd (& self) -> Option < c_int > { match * self { ChildStdio :: Inherit => None , ChildStdio :: Explicit (fd) => Some (fd) , ChildStdio :: Owned (ref fd) => Some (fd . as_raw_fd ()) , # [cfg (target_os = "fuchsia")] ChildStdio :: Null => None , } } }}}
mkitem!{mkimpl!{impl fmt :: Debug for Command { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if f . alternate () { let mut debug_command = f . debug_struct ("Command") ; debug_command . field ("program" , & self . program) . field ("args" , & self . args) ; if ! self . env . is_unchanged () { debug_command . field ("env" , & self . env) ; } if self . cwd . is_some () { debug_command . field ("cwd" , & self . cwd) ; } if self . uid . is_some () { debug_command . field ("uid" , & self . uid) ; } if self . gid . is_some () { debug_command . field ("gid" , & self . gid) ; } if self . groups . is_some () { debug_command . field ("groups" , & self . groups) ; } if self . stdin . is_some () { debug_command . field ("stdin" , & self . stdin) ; } if self . stdout . is_some () { debug_command . field ("stdout" , & self . stdout) ; } if self . stderr . is_some () { debug_command . field ("stderr" , & self . stderr) ; } if self . pgroup . is_some () { debug_command . field ("pgroup" , & self . pgroup) ; } # [cfg (target_os = "linux")] { debug_command . field ("create_pidfd" , & self . create_pidfd) ; } debug_command . finish () } else { if let Some (ref cwd) = self . cwd { write ! (f , "cd {cwd:?} && ") ? ; } if self . env . does_clear () { write ! (f , "env -i ") ? ; } else { let mut any_removed = false ; for (key , value_opt) in self . get_envs () { if value_opt . is_none () { if ! any_removed { write ! (f , "env ") ? ; any_removed = true ; } write ! (f , "-u {} " , key . to_string_lossy ()) ? ; } } } for (key , value_opt) in self . get_envs () { if let Some (value) = value_opt { write ! (f , "{}={value:?} " , key . to_string_lossy ()) ? ; } } if * self . program != self . args [0] { write ! (f , "[{:?}] " , self . program) ? ; } write ! (f , "{:?}" , & self . args [0]) ? ; for arg in self . get_args () { write ! (f , " {:?}" , arg) ? ; } Ok (()) } } }}}
mkitem!{mkstruct!{# [derive (PartialEq , Eq , Clone , Copy)] pub struct ExitCode (u8) ;}}
mkitem!{mkimpl!{impl fmt :: Debug for ExitCode { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("unix_exit_status") . field (& self . 0) . finish () } }}}
mkitem!{mkimpl!{impl ExitCode { pub const SUCCESS : ExitCode = ExitCode (EXIT_SUCCESS as _) ; pub const FAILURE : ExitCode = ExitCode (EXIT_FAILURE as _) ; # [inline] pub fn as_i32 (& self) -> i32 { self . 0 as i32 } }}}
mkitem!{mkimpl!{impl From < u8 > for ExitCode { fn from (code : u8) -> Self { Self (code) } }}}
mkitem!{mkstruct!{pub struct CommandArgs < 'a > { iter : CStringIter < 'a > , }}}
mkitem!{mkimpl!{impl < 'a > Iterator for CommandArgs < 'a > { type Item = & 'a OsStr ; fn next (& mut self) -> Option < & 'a OsStr > { self . iter . next () . map (| cs | OsStr :: from_bytes (cs . to_bytes ())) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }}}
mkitem!{mkimpl!{impl < 'a > ExactSizeIterator for CommandArgs < 'a > { fn len (& self) -> usize { self . iter . len () } fn is_empty (& self) -> bool { self . iter . is_empty () } }}}
mkitem!{mkimpl!{impl < 'a > fmt :: Debug for CommandArgs < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter . clone ()) . finish () } }}}