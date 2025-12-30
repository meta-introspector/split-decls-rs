// Generated macro for test (module)
macro_rules! Depcrate_eventstest {
() => {
// Module: crate::events
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use pretty_assertions :: assert_eq ; # [test] fn bytestart_create () { let b = BytesStart :: new ("test") ; assert_eq ! (b . len () , 4) ; assert_eq ! (b . name () , QName (b"test")) ; } # [test] fn bytestart_set_name () { let mut b = BytesStart :: new ("test") ; assert_eq ! (b . len () , 4) ; assert_eq ! (b . name () , QName (b"test")) ; assert_eq ! (b . attributes_raw () , b"") ; b . push_attribute (("x" , "a")) ; assert_eq ! (b . len () , 10) ; assert_eq ! (b . attributes_raw () , b" x=\"a\"") ; b . set_name (b"g") ; assert_eq ! (b . len () , 7) ; assert_eq ! (b . name () , QName (b"g")) ; } # [test] fn bytestart_clear_attributes () { let mut b = BytesStart :: new ("test") ; b . push_attribute (("x" , "y\"z")) ; b . push_attribute (("x" , "y\"z")) ; b . clear_attributes () ; assert ! (b . attributes () . next () . is_none ()) ; assert_eq ! (b . len () , 4) ; assert_eq ! (b . name () , QName (b"test")) ; } }
};
}
