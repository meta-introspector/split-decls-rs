// Generated macro for properties_get_all_async (function)
macro_rules! Depcrate_testproperties_get_all_async {
() => {
// Module: crate::test
// Provides: {"properties_get_all_async"}
// Dependencies: {}
# [tokio :: test] async fn properties_get_all_async () { use dbus :: channel :: MatchingReceiver ; let (resource , bus) = dbus_tokio :: connection :: new_session_sync () . unwrap () ; tokio :: spawn (async { resource . await ; }) ; bus . request_name ("com.example.dbusrs.properties" , false , false , false) . await . unwrap () ; let mut cr = Crossroads :: new () ; let spawner = Box :: new (| fut | { tokio :: spawn (fut) ; }) ; cr . set_async_support (Some ((bus . clone () , spawner))) ; let iface = cr . register ("com.example.dbusrs.properties" , | b | { b . property ("Sync") . get (| _ , _ | Ok (1)) ; b . property ("Async") . get_async (| mut ctx , _ | async move { ctx . reply (Ok (2)) }) ; }) ; cr . insert ("/" , & [iface] , ()) ; bus . start_receive (dbus :: message :: MatchRule :: new_method_call () , Box :: new (move | msg , conn | { cr . handle_message (msg , conn) . unwrap () ; true })) ; let proxy = dbus :: nonblock :: Proxy :: new ("com.example.dbusrs.properties" , "/" , Duration :: from_secs (3600) , bus) ; let (response ,) : (HashMap < String , Variant < Box < dyn RefArg > > > ,) = proxy . method_call ("org.freedesktop.DBus.Properties" , "GetAll" , ("com.example.dbusrs.properties" ,)) . await . unwrap () ; assert_eq ! (response . get ("Sync") . unwrap () . as_i64 () , Some (1)) ; assert_eq ! (response . get ("Async") . unwrap () . as_i64 () , Some (2)) ; assert_eq ! (response . len () , 2) ; }
};
}
