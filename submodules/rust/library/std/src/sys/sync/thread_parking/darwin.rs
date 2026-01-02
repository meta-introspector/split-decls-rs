mkuse!{use crate :: pin :: Pin ;}
mkuse!{use crate :: sync :: atomic :: Ordering :: { Acquire , Release } ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicI8 } ;}
mkuse!{use crate :: time :: Duration ;}
mkitem!{type dispatch_semaphore_t = * mut crate :: ffi :: c_void ;}
mkitem!{type dispatch_time_t = u64 ;}
mkitem!{const DISPATCH_TIME_NOW : dispatch_time_t = 0 ;}
mkitem!{const DISPATCH_TIME_FOREVER : dispatch_time_t = ! 0 ;}
mkitem!{unsafe extern "C" { fn dispatch_time (when : dispatch_time_t , delta : i64) -> dispatch_time_t ; fn dispatch_semaphore_create (val : isize) -> dispatch_semaphore_t ; fn dispatch_semaphore_wait (dsema : dispatch_semaphore_t , timeout : dispatch_time_t) -> isize ; fn dispatch_semaphore_signal (dsema : dispatch_semaphore_t) -> isize ; fn dispatch_release (object : * mut crate :: ffi :: c_void) ; }}
mkitem!{const EMPTY : i8 = 0 ;}
mkitem!{const NOTIFIED : i8 = 1 ;}
mkitem!{const PARKED : i8 = - 1 ;}
mkitem!{mkstruct!{pub struct Parker { semaphore : dispatch_semaphore_t , state : Atomic < i8 > , }}}
mkitem!{mkimpl!{unsafe impl Sync for Parker { }}}
mkitem!{mkimpl!{unsafe impl Send for Parker { }}}
mkitem!{mkimpl!{impl Parker { pub unsafe fn new_in_place (parker : * mut Parker) { let semaphore = dispatch_semaphore_create (0) ; assert ! (! semaphore . is_null () , "failed to create dispatch semaphore for thread synchronization") ; parker . write (Parker { semaphore , state : AtomicI8 :: new (EMPTY) }) } pub unsafe fn park (self : Pin < & Self >) { if self . state . fetch_sub (1 , Acquire) == NOTIFIED { return ; } while dispatch_semaphore_wait (self . semaphore , DISPATCH_TIME_FOREVER) != 0 { } self . state . swap (EMPTY , Acquire) ; } pub unsafe fn park_timeout (self : Pin < & Self > , dur : Duration) { if self . state . fetch_sub (1 , Acquire) == NOTIFIED { return ; } let nanos = dur . as_nanos () . try_into () . unwrap_or (i64 :: MAX) ; let timeout = dispatch_time (DISPATCH_TIME_NOW , nanos) ; let timeout = dispatch_semaphore_wait (self . semaphore , timeout) != 0 ; let state = self . state . swap (EMPTY , Acquire) ; if state == NOTIFIED && timeout { while dispatch_semaphore_wait (self . semaphore , DISPATCH_TIME_FOREVER) != 0 { } } else { } } pub fn unpark (self : Pin < & Self >) { let state = self . state . swap (NOTIFIED , Release) ; if state == PARKED { unsafe { dispatch_semaphore_signal (self . semaphore) ; } } } }}}
mkitem!{mkimpl!{impl Drop for Parker { fn drop (& mut self) { unsafe { dispatch_release (self . semaphore) ; } } }}}