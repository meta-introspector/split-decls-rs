// Generated macro for channel_simple_test (function)
macro_rules! Depcrate_channelchannel_simple_test {
() => {
// Module: crate::channel
// Provides: {"channel_simple_test"}
// Dependencies: {}
# [test] fn channel_simple_test () { let mut c = Channel :: get_private (BusType :: Session) . unwrap () ; assert ! (c . is_connected ()) ; c . set_watch_enabled (true) ; let fd = c . watch () ; println ! ("{:?}" , fd) ; let m = Message :: new_method_call ("org.freedesktop.DBus" , "/" , "org.freedesktop.DBus" , "ListNames") . unwrap () ; let reply = c . send (m) . unwrap () ; let my_name = c . unique_name () . unwrap () ; loop { while let Some (mut msg) = c . pop_message () { println ! ("{:?}" , msg) ; if msg . get_reply_serial () == Some (reply) { let r = msg . as_result () . unwrap () ; let z : crate :: arg :: Array < & str , _ > = r . get1 () . unwrap () ; for n in z { println ! ("{}" , n) ; if n == my_name { return ; } } assert ! (false) ; } else if let Some (r) = default_reply (& msg) { c . send (r) . unwrap () ; } } c . read_write (Some (std :: time :: Duration :: from_millis (100))) . unwrap () ; } }
};
}
