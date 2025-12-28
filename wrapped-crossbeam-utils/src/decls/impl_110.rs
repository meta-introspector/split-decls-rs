macro_rules! deps {
    () => {
        Inner!();
        UnparkReason!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl Inner { fn park (& self , deadline : Option < Instant >) -> UnparkReason { if self . state . compare_exchange (NOTIFIED , EMPTY , SeqCst , SeqCst) . is_ok () { return UnparkReason :: Unparked ; } if let Some (deadline) = deadline { if deadline <= Instant :: now () { return UnparkReason :: Timeout ; } } let mut m = self . lock . lock () . unwrap () ; match self . state . compare_exchange (EMPTY , PARKED , SeqCst , SeqCst) { Ok (_) => { } Err (NOTIFIED) => { let old = self . state . swap (EMPTY , SeqCst) ; assert_eq ! (old , NOTIFIED , "park state changed unexpectedly") ; return UnparkReason :: Unparked ; } Err (n) => panic ! ("inconsistent park_timeout state: {}" , n) , } loop { m = match deadline { None => self . cvar . wait (m) . unwrap () , Some (deadline) => { let now = Instant :: now () ; if now < deadline { self . cvar . wait_timeout (m , deadline - now) . unwrap () . 0 } else { return match self . state . swap (EMPTY , SeqCst) { NOTIFIED => UnparkReason :: Unparked , PARKED => UnparkReason :: Timeout , n => panic ! ("inconsistent park_timeout state: {}" , n) , } ; } } } ; if self . state . compare_exchange (NOTIFIED , EMPTY , SeqCst , SeqCst) . is_ok () { return UnparkReason :: Unparked ; } } } pub (crate) fn unpark (& self) { match self . state . swap (NOTIFIED , SeqCst) { EMPTY => return , NOTIFIED => return , PARKED => { } _ => panic ! ("inconsistent state in unpark") , } drop (self . lock . lock () . unwrap ()) ; self . cvar . notify_one () ; } }
    };
}

impl_110!();