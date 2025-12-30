// Generated macro for deref (function)
macro_rules! Depcratederef {
() => {
// Module: crate
// Provides: {"deref"}
// Dependencies: {}
# [test] fn deref () { use std :: string :: String ; fn is_str (_ : & str) { } let value : Either < String , & str > = Left (String :: from ("test")) ; is_str (& value) ; }
};
}
