// Generated macro for impl_134 (impl)
macro_rules! Depcrateimpl_134 {
() => {
// Module: crate
// Provides: {"impl_134"}
// Dependencies: {}
impl fmt :: Display for ByteClass { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { try ! (write ! (f , "(?-u:[")) ; for range in self . iter () { if range . start == b'-' || range . end == b'-' { try ! (write ! (f , "-")) ; break ; } } for range in self . iter () { let mut range = * range ; if range . start == b'-' { range . start += 1 ; } if range . end == b'-' { range . start -= 1 ; } if range . start > range . end { continue ; } try ! (write ! (f , "{}" , range)) ; } try ! (write ! (f , "])")) ; Ok (()) } }
};
}
