// Generated macro for staticize (function)
macro_rules! Depcratestaticize {
() => {
// Module: crate
// Provides: {"staticize"}
// Dependencies: {}
fn staticize (generics : & Generics) -> Generics { let mut ret = generics . clone () ; for lt in ret . lifetimes_mut () { lt . lifetime = Lifetime :: new ("'static" , Span :: call_site ()) ; } ; ret }
};
}
