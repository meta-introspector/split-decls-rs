// Generated macro for object_manager_root (function)
macro_rules! Depcrate_testobject_manager_root {
() => {
// Module: crate::test
// Provides: {"object_manager_root"}
// Dependencies: {}
# [test] fn object_manager_root () { let bus = dbus :: blocking :: Connection :: new_session () . unwrap () ; bus . request_name ("com.example.dbusrs.objmgr_root" , false , false , false) . unwrap () ; let mut cr = Crossroads :: new () ; cr . set_object_manager_support (Some (std :: sync :: Arc :: new (std :: sync :: Mutex :: new (vec ! ())))) ; cr . insert ("/" , & [cr . object_manager ()] , ()) ; cr . insert ("/foo" , & [] , ()) ; use dbus :: channel :: MatchingReceiver ; let shared_cr = std :: sync :: Arc :: new (std :: sync :: Mutex :: new (cr)) ; let altcr = shared_cr . clone () ; bus . start_receive (dbus :: message :: MatchRule :: new_method_call () , Box :: new (move | msg , conn | { altcr . lock () . unwrap () . handle_message (msg , conn) . unwrap () ; true })) ; let service_thread = std :: thread :: spawn (move | | { bus . process (std :: time :: Duration :: new (u64 :: MAX , 0)) . unwrap () ; }) ; let msg = Message :: new_method_call ("com.example.dbusrs.objmgr_root" , "/" , "org.freedesktop.DBus.ObjectManager" , "GetManagedObjects") . unwrap () ; dispatch_helper (& mut shared_cr . lock () . unwrap () , msg) ; service_thread . join () . unwrap () ; }
};
}
