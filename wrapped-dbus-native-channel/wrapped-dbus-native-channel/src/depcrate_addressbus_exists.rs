// Generated macro for bus_exists (function)
macro_rules! Depcrate_addressbus_exists {
() => {
// Module: crate::address
// Provides: {"bus_exists"}
// Dependencies: {}
# [test] fn bus_exists () { let addr = read_session_address () . unwrap () ; println ! ("Bus address is: {:?}" , addr) ; if addr . starts_with ("unix:path=") { let path = std :: path :: Path :: new (& addr ["unix:path=" . len () ..]) ; assert ! (path . exists ()) ; } let addr = read_system_address () . unwrap () ; if addr . starts_with ("unix:path=") { let path = std :: path :: Path :: new (& addr ["unix:path=" . len () ..]) ; assert ! (path . exists ()) ; } }
};
}
