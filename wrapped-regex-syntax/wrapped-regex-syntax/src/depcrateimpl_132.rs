// Generated macro for impl_132 (impl)
macro_rules! Depcrateimpl_132 {
() => {
// Module: crate
// Provides: {"impl_132"}
// Dependencies: {}
impl fmt :: Display for CharClass { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { try ! (write ! (f , "(?u:[")) ; for range in self . iter () { if range . start == '-' || range . end == '-' { try ! (write ! (f , "-")) ; break ; } } for range in self . iter () { let mut range = * range ; if range . start == '-' { range . start = ((range . start as u8) + 1) as char ; } if range . end == '-' { range . end = ((range . end as u8) - 1) as char ; } if range . start > range . end { continue ; } try ! (write ! (f , "{}" , range)) ; } try ! (write ! (f , "])")) ; Ok (()) } }
};
}
