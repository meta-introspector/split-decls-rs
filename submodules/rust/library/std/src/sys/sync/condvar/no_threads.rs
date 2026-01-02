mkuse!{use crate :: sys :: sync :: Mutex ;}
mkuse!{use crate :: thread :: sleep ;}
mkuse!{use crate :: time :: Duration ;}
mkitem!{mkstruct!{pub struct Condvar { }}}
mkitem!{mkimpl!{impl Condvar { # [inline] pub const fn new () -> Condvar { Condvar { } } # [inline] pub fn notify_one (& self) { } # [inline] pub fn notify_all (& self) { } pub unsafe fn wait (& self , _mutex : & Mutex) { panic ! ("condvar wait not supported") } pub unsafe fn wait_timeout (& self , _mutex : & Mutex , dur : Duration) -> bool { sleep (dur) ; false } }}}