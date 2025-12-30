// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl < TLeft , TRight > Display for Comparison < '_ , TLeft , TRight > where TLeft : Debug + ? Sized , TRight : Debug + ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let left_debug = format ! ("{:#?}" , self . left) ; let right_debug = format ! ("{:#?}" , self . right) ; printer :: write_header (f) ? ; printer :: write_lines (f , & left_debug , & right_debug) } }
};
}
