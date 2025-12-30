// Generated macro for test_peer (function)
macro_rules! Depcrate_blockingtest_peer {
() => {
// Module: crate::blocking
// Provides: {"test_peer"}
// Dependencies: {}
# [test] fn test_peer () { let c = Connection :: new_session () . unwrap () ; let c_name = c . unique_name () . into_static () ; use std :: sync :: Arc ; let done = Arc :: new (false) ; let d2 = done . clone () ; let j = std :: thread :: spawn (move | | { let c2 = Connection :: new_session () . unwrap () ; let proxy = c2 . with_proxy (c_name , "/" , Duration :: from_secs (5)) ; let (s2 ,) : (String ,) = proxy . method_call ("org.freedesktop.DBus.Peer" , "GetMachineId" , ()) . unwrap () ; println ! ("{}" , s2) ; assert_eq ! (Arc :: strong_count (& d2) , 2) ; s2 }) ; assert_eq ! (Arc :: strong_count (& done) , 2) ; for _ in 0 .. 30 { c . process (Duration :: from_millis (100)) . unwrap () ; if Arc :: strong_count (& done) < 2 { break ; } } let s2 = j . join () . unwrap () ; # [cfg (unix)] { let proxy = c . with_proxy ("org.a11y.Bus" , "/org/a11y/bus" , Duration :: from_secs (5)) ; let (s1 ,) : (String ,) = proxy . method_call ("org.freedesktop.DBus.Peer" , "GetMachineId" , ()) . unwrap () ; assert_eq ! (s1 , s2) ; } }
};
}
