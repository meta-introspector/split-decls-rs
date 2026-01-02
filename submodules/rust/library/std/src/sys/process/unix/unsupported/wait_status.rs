mkuse!{use super :: ExitStatusError ;}
mkuse!{use crate :: ffi :: c_int ;}
mkuse!{use crate :: fmt ;}
mkuse!{use crate :: num :: NonZero ;}
mkitem!{mkstruct!{# [doc = " Emulated wait status for use by `unsupported.rs`"] # [doc = ""] # [doc = " Uses the \"traditional unix\" encoding.  For use on platfors which are `#[cfg(unix)]`"] # [doc = " but do not actually support subprocesses at all."] # [doc = ""] # [doc = " These platforms aren't Unix, but are simply pretending to be for porting convenience."] # [doc = " So, we provide a faithful pretence here."] # [derive (PartialEq , Eq , Clone , Copy , Debug , Default)] pub struct ExitStatus { wait_status : c_int , }}}
mkitem!{mkimpl!{# [doc = " Converts a raw `c_int` to a type-safe `ExitStatus` by wrapping it"] impl From < c_int > for ExitStatus { fn from (wait_status : c_int) -> ExitStatus { ExitStatus { wait_status } } }}}
mkitem!{mkimpl!{impl fmt :: Display for ExitStatus { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "emulated wait status: {}" , self . wait_status) } }}}
mkitem!{mkimpl!{impl ExitStatus { pub fn code (& self) -> Option < i32 > { let w = self . wait_status ; if (w & 0x7f) == 0 { Some ((w & 0xff00) >> 8) } else { None } } # [allow (unused)] pub fn exit_ok (& self) -> Result < () , ExitStatusError > { match NonZero :: try_from (self . wait_status) { Ok (failure) => Err (ExitStatusError (failure)) , Err (_) => Ok (()) , } } pub fn signal (& self) -> Option < i32 > { let signal = self . wait_status & 0x007f ; if signal > 0 && signal < 0x7f { Some (signal) } else { None } } pub fn core_dumped (& self) -> bool { self . signal () . is_some () && (self . wait_status & 0x80) != 0 } pub fn stopped_signal (& self) -> Option < i32 > { let w = self . wait_status ; if (w & 0xff) == 0x7f { Some ((w & 0xff00) >> 8) } else { None } } pub fn continued (& self) -> bool { self . wait_status == 0xffff } pub fn into_raw (& self) -> c_int { self . wait_status } }}}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}