mkuse!{use crate :: cell :: UnsafeCell ;}
mkuse!{use crate :: pin :: Pin ;}
mkuse!{use crate :: sync :: atomic :: Ordering :: { Acquire , Relaxed , Release } ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicI8 , fence } ;}
mkuse!{use crate :: sys :: thread_parking :: { ThreadId , current , park , park_timeout , unpark } ;}
mkuse!{use crate :: time :: Duration ;}
mkitem!{mkstruct!{pub struct Parker { state : Atomic < i8 > , tid : UnsafeCell < Option < ThreadId > > , }}}
mkitem!{const PARKED : i8 = - 1 ;}
mkitem!{const EMPTY : i8 = 0 ;}
mkitem!{const NOTIFIED : i8 = 1 ;}
mkitem!{mkimpl!{impl Parker { pub fn new () -> Parker { Parker { state : AtomicI8 :: new (EMPTY) , tid : UnsafeCell :: new (None) } } # [doc = " Creates a new thread parker. UNIX requires this to happen in-place."] pub unsafe fn new_in_place (parker : * mut Parker) { parker . write (Parker :: new ()) } # [doc = " # Safety"] # [doc = " * must always be called from the same thread"] # [doc = " * must be called before the state is set to PARKED"] unsafe fn init_tid (& self) { if self . tid . get () . read () . is_none () { self . tid . get () . write (Some (current ())) ; fence (Release) ; } } pub unsafe fn park (self : Pin < & Self >) { self . init_tid () ; let state = self . state . fetch_sub (1 , Acquire) ; if state == EMPTY { while self . state . compare_exchange (NOTIFIED , EMPTY , Acquire , Relaxed) . is_err () { park (self . state . as_ptr () . addr ()) ; } } } pub unsafe fn park_timeout (self : Pin < & Self > , dur : Duration) { self . init_tid () ; let state = self . state . fetch_sub (1 , Acquire) . wrapping_sub (1) ; if state == PARKED { park_timeout (dur , self . state . as_ptr () . addr ()) ; self . state . swap (EMPTY , Acquire) ; } } pub fn unpark (self : Pin < & Self >) { let state = self . state . swap (NOTIFIED , Release) ; if state == PARKED { fence (Acquire) ; let tid = unsafe { self . tid . get () . read () . unwrap_unchecked () } ; unpark (tid , self . state . as_ptr () . addr ()) ; } } }}}
mkitem!{mkimpl!{unsafe impl Send for Parker { }}}
mkitem!{mkimpl!{unsafe impl Sync for Parker { }}}