// Generated macro for hex (function)
macro_rules! Depcratehex {
() => {
// Module: crate
// Provides: {"hex"}
// Dependencies: {}
fn hex < 'a > (f : & mut fmt :: Formatter < '_ > , payload : impl IntoIterator < Item = & 'a u8 >) -> fmt :: Result { for (i , b) in payload . into_iter () . enumerate () { if i == 0 { write ! (f , "0x") ? ; } write ! (f , "{b:02x}") ? ; } Ok (()) }
};
}
