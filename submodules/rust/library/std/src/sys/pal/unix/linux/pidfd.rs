mkuse!{use crate :: io ;}
mkuse!{use crate :: os :: fd :: { AsRawFd , FromRawFd , RawFd } ;}
mkuse!{use crate :: sys :: cvt ;}
mkuse!{use crate :: sys :: fd :: FileDesc ;}
mkuse!{use crate :: sys :: process :: ExitStatus ;}
mkuse!{use crate :: sys_common :: { AsInner , FromInner , IntoInner } ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkitem!{mkstruct!{# [derive (Debug)] pub (crate) struct PidFd (FileDesc) ;}}
mkitem!{mkimpl!{impl PidFd { pub fn kill (& self) -> io :: Result < () > { self . send_signal (libc :: SIGKILL) } pub (crate) fn send_signal (& self , signal : i32) -> io :: Result < () > { cvt (unsafe { libc :: syscall (libc :: SYS_pidfd_send_signal , self . 0 . as_raw_fd () , signal , crate :: ptr :: null :: < () > () , 0 ,) }) . map (drop) } pub fn wait (& self) -> io :: Result < ExitStatus > { let mut siginfo : libc :: siginfo_t = unsafe { crate :: mem :: zeroed () } ; cvt (unsafe { libc :: waitid (libc :: P_PIDFD , self . 0 . as_raw_fd () as u32 , & mut siginfo , libc :: WEXITED) }) ? ; Ok (ExitStatus :: from_waitid_siginfo (siginfo)) } pub fn try_wait (& self) -> io :: Result < Option < ExitStatus > > { let mut siginfo : libc :: siginfo_t = unsafe { crate :: mem :: zeroed () } ; cvt (unsafe { libc :: waitid (libc :: P_PIDFD , self . 0 . as_raw_fd () as u32 , & mut siginfo , libc :: WEXITED | libc :: WNOHANG ,) }) ? ; if unsafe { siginfo . si_pid () } == 0 { Ok (None) } else { Ok (Some (ExitStatus :: from_waitid_siginfo (siginfo))) } } }}}
mkitem!{mkimpl!{impl AsInner < FileDesc > for PidFd { fn as_inner (& self) -> & FileDesc { & self . 0 } }}}
mkitem!{mkimpl!{impl IntoInner < FileDesc > for PidFd { fn into_inner (self) -> FileDesc { self . 0 } }}}
mkitem!{mkimpl!{impl FromInner < FileDesc > for PidFd { fn from_inner (inner : FileDesc) -> Self { Self (inner) } }}}
mkitem!{mkimpl!{impl FromRawFd for PidFd { unsafe fn from_raw_fd (fd : RawFd) -> Self { Self (FileDesc :: from_raw_fd (fd)) } }}}