// Generated macro for peer (function)
macro_rules! Depcrate_testpeer {
() => {
// Module: crate::test
// Provides: {"peer"}
// Dependencies: {}
# [test] fn peer () { let mut cr = Crossroads :: new () ; let msg = Message :: new_method_call ("com.example.dbusrs.peer" , "/" , "org.freedesktop.DBus.Peer" , "GetMachineId") . unwrap () ; let r = dispatch_helper (& mut cr , msg) ; let mid : & str = r . read1 () . unwrap () ; println ! ("{}" , mid) ; }
};
}
