// Generated macro for properties_get_all (function)
macro_rules! Depcrate_testproperties_get_all {
() => {
// Module: crate::test
// Provides: {"properties_get_all"}
// Dependencies: {}
# [test] fn properties_get_all () { let bus = dbus :: blocking :: Connection :: new_session () . unwrap () ; bus . request_name ("com.example.dbusrs.properties" , false , false , false) . unwrap () ; let mut cr = Crossroads :: new () ; let iface = cr . register ("com.example.dbusrs.properties" , | b | { b . property ("One") . get (| _ , _ | Ok (1)) ; b . property ("Two") . get (| _ , _ | Ok (2)) ; }) ; cr . insert ("/" , & [iface] , ()) ; let msg = Message :: call_with_args ("com.example.dbusrs.properties" , "/" , "org.freedesktop.DBus.Properties" , "GetAll" , ("com.example.dbusrs.properties" ,) ,) ; let r = dispatch_helper (& mut cr , msg) ; let response : HashMap < String , Variant < Box < dyn RefArg > > > = r . read1 () . unwrap () ; assert_eq ! (response . get ("One") . unwrap () . as_i64 () , Some (1)) ; assert_eq ! (response . get ("Two") . unwrap () . as_i64 () , Some (2)) ; assert_eq ! (response . len () , 2) ; }
};
}
