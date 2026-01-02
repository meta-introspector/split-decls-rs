mkuse!{use libc :: { c_int , pid_t } ;}
mkuse!{use super :: common :: * ;}
mkuse!{use crate :: io ;}
mkuse!{use crate :: num :: NonZero ;}
mkuse!{use crate :: sys :: pal :: unsupported :: * ;}
mkitem!{mkimpl!{impl Command { pub fn spawn (& mut self , _default : Stdio , _needs_stdin : bool ,) -> io :: Result < (Process , StdioPipes) > { unsupported () } pub fn exec (& mut self , _default : Stdio) -> io :: Error { unsupported_err () } }}}

macro_rules! output_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function output in module {}", module_path!());
    };
}

mkfn!{
    output_introspect!();
    pub fn output (_ : & mut Command) -> io :: Result < (ExitStatus , Vec < u8 > , Vec < u8 >) > { unsupported () }
}
mkitem!{mkstruct!{pub struct Process { _handle : pid_t , }}}
mkitem!{mkimpl!{impl Process { pub fn id (& self) -> u32 { 0 } pub fn kill (& self) -> io :: Result < () > { unsupported () } pub fn send_signal (& self , _signal : i32) -> io :: Result < () > { unsupported () } pub fn wait (& mut self) -> io :: Result < ExitStatus > { unsupported () } pub fn try_wait (& mut self) -> io :: Result < Option < ExitStatus > > { unsupported () } }}}
mkmod!{wait_status, { 
                getname!(wait_status);
                getsrc!(wait_status);
                getpath!(wait_status);
                get_deps!(wait_status);
                get_crates!(wait_status);
                mkinclude!(wait_status);
                 
            }}
mkuse!{pub use wait_status :: ExitStatus ;}
mkitem!{mkstruct!{# [derive (PartialEq , Eq , Clone , Copy , Debug)] pub struct ExitStatusError (NonZero < c_int >) ;}}
mkitem!{mkimpl!{impl Into < ExitStatus > for ExitStatusError { fn into (self) -> ExitStatus { ExitStatus :: from (c_int :: from (self . 0)) } }}}
mkitem!{mkimpl!{impl ExitStatusError { pub fn code (self) -> Option < NonZero < i32 > > { ExitStatus :: from (c_int :: from (self . 0)) . code () . map (| st | st . try_into () . unwrap ()) } }}}