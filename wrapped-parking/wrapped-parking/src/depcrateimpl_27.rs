// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl Inner { fn park (& self , timeout : Option < Duration >) -> bool { if self . state . compare_exchange (NOTIFIED , EMPTY , SeqCst , SeqCst) . is_ok () { return true ; } if let Some (dur) = timeout { if dur == Duration :: from_millis (0) { return false ; } } let mut m = self . lock . lock () . unwrap () ; match self . state . compare_exchange (EMPTY , PARKED , SeqCst , SeqCst) { Ok (_) => { } Err (NOTIFIED) => { let old = self . state . swap (EMPTY , SeqCst) ; assert_eq ! (old , NOTIFIED , "park state changed unexpectedly") ; return true ; } Err (n) => panic ! ("inconsistent park_timeout state: {}" , n) , } match timeout { None => { loop { m = self . cvar . wait (m) . unwrap () ; if self . state . compare_exchange (NOTIFIED , EMPTY , SeqCst , SeqCst) . is_ok () { return true ; } } } Some (timeout) => { # [cfg (not (loom))] { let (_m , _result) = self . cvar . wait_timeout (m , timeout) . unwrap () ; match self . state . swap (EMPTY , SeqCst) { NOTIFIED => true , PARKED => false , n => panic ! ("inconsistent park_timeout state: {}" , n) , } } # [cfg (loom)] { let _ = timeout ; panic ! ("park_timeout is not supported under loom") ; } } } } pub fn unpark (& self) -> bool { match self . state . swap (NOTIFIED , SeqCst) { EMPTY => return true , NOTIFIED => return false , PARKED => { } _ => panic ! ("inconsistent state in unpark") , } drop (self . lock . lock () . unwrap ()) ; self . cvar . notify_one () ; true } }
};
}
