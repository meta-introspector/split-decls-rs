// Generated macro for type_conversions (function)
macro_rules! Depcratetype_conversions {
() => {
// Module: crate
// Provides: {"type_conversions"}
// Dependencies: {}
# [test] fn type_conversions () { use std :: borrow :: Cow ; let x : & ObjectPath = ObjectPath :: new ("/test") . unwrap () ; let y : ObjectPathBuf = ObjectPath :: new_owned ("/test") . unwrap () ; assert_eq ! (x , &* y) ; let x = Cow :: from (x) ; let y = Cow :: from (y) ; assert_eq ! (x , y) ; let x : & DBusStr = (& * x) . into () ; let y = DBusString :: from (y . into_owned ()) ; assert_eq ! (x , &* y) ; }
};
}
