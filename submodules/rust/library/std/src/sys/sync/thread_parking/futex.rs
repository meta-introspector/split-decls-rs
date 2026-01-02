mkuse!{use crate :: pin :: Pin ;}
mkuse!{use crate :: sync :: atomic :: Ordering :: { Acquire , Release } ;}
mkuse!{use crate :: sys :: futex :: { self , futex_wait , futex_wake } ;}
mkuse!{use crate :: time :: Duration ;}
mkitem!{type Futex = futex :: SmallFutex ;}
mkitem!{type State = futex :: SmallPrimitive ;}
mkitem!{const PARKED : State = State :: MAX ;}
mkitem!{const EMPTY : State = 0 ;}
mkitem!{const NOTIFIED : State = 1 ;}
mkitem!{mkstruct!{pub struct Parker { state : Futex , }}}
mkitem!{mkimpl!{impl Parker { # [doc = " Constructs the futex parker. The UNIX parker implementation"] # [doc = " requires this to happen in-place."] pub unsafe fn new_in_place (parker : * mut Parker) { unsafe { parker . write (Self { state : Futex :: new (EMPTY) }) } ; } pub unsafe fn park (self : Pin < & Self >) { if self . state . fetch_sub (1 , Acquire) == NOTIFIED { return ; } loop { futex_wait (& self . state , PARKED , None) ; if self . state . compare_exchange (NOTIFIED , EMPTY , Acquire , Acquire) . is_ok () { return ; } else { } } } pub unsafe fn park_timeout (self : Pin < & Self > , timeout : Duration) { if self . state . fetch_sub (1 , Acquire) == NOTIFIED { return ; } futex_wait (& self . state , PARKED , Some (timeout)) ; if self . state . swap (EMPTY , Acquire) == NOTIFIED { } else { } } # [inline] pub fn unpark (self : Pin < & Self >) { if self . state . swap (NOTIFIED , Release) == PARKED { futex_wake (& self . state) ; } } }}}