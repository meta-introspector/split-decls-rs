// Generated macro for test (module)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { # [test] fn test_size () { println ! ("String: {}" , std :: mem :: size_of ::< crate :: string :: StdString > ()) ; println ! ("Box<str>: {}" , std :: mem :: size_of ::< crate :: backend :: DefaultStr > ()) ; println ! ("Box<Box<str>>: {}" , std :: mem :: size_of ::< Box < crate :: backend :: DefaultStr >> ()) ; println ! ("str: {}" , std :: mem :: size_of ::<&'static str > ()) ; println ! ("Cow: {}" , std :: mem :: size_of ::< std :: borrow :: Cow <'static , str >> ()) ; } }
};
}
