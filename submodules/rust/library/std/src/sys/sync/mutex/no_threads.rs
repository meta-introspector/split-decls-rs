mkuse!{use crate :: cell :: Cell ;}
mkitem!{mkstruct!{pub struct Mutex { locked : Cell < bool > , }}}
mkitem!{mkimpl!{unsafe impl Send for Mutex { }}}
mkitem!{mkimpl!{unsafe impl Sync for Mutex { }}}
mkitem!{mkimpl!{impl Mutex { # [inline] pub const fn new () -> Mutex { Mutex { locked : Cell :: new (false) } } # [inline] pub fn lock (& self) { assert_eq ! (self . locked . replace (true) , false , "cannot recursively acquire mutex") ; } # [inline] pub unsafe fn unlock (& self) { self . locked . set (false) ; } # [inline] pub fn try_lock (& self) -> bool { self . locked . replace (true) == false } }}}