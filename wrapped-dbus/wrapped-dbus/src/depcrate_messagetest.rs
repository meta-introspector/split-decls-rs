// Generated macro for test (module)
macro_rules! Depcrate_messagetest {
() => {
// Module: crate::message
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: { Message } ; use crate :: strings :: BusName ; # [test] fn set_valid_destination () { let mut m = Message :: new_method_call ("org.test.rust" , "/" , "org.test.rust" , "Test") . unwrap () ; let d = Some (BusName :: new (":1.14") . unwrap ()) ; m . set_destination (d) ; assert ! (! m . get_no_reply ()) ; m . set_no_reply (true) ; assert ! (m . get_no_reply ()) ; } # [test] fn set_valid_sender () { let mut m = Message :: new_method_call ("org.test.rust" , "/" , "org.test.rust" , "Test") . unwrap () ; let sender = ":1.14" ; let d = Some (BusName :: new (sender) . unwrap ()) ; m . set_sender (d) ; assert_eq ! (sender , m . sender () . unwrap () . to_string ()) ; } # [test] fn marshal () { let mut m = Message :: new_method_call ("org.freedesktop.DBus" , "/org/freedesktop/DBus" , "org.freedesktop.DBus" , "Hello") . unwrap () ; m . set_serial (1) ; let r = m . marshal (| d | { let m2 = Message :: demarshal (d) . unwrap () ; assert_eq ! (&* m2 . path () . unwrap () , "/org/freedesktop/DBus") ; Err (45) }) ; assert_eq ! (45 , r . unwrap_err ()) ; } }
};
}
