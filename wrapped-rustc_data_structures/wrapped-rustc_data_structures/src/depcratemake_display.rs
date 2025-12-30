// Generated macro for make_display (function)
macro_rules! Depcratemake_display {
() => {
// Module: crate
// Provides: {"make_display"}
// Dependencies: {}
# [doc = " Turns a closure that takes an `&mut Formatter` into something that can be display-formatted."] pub fn make_display (f : impl Fn (& mut fmt :: Formatter < '_ >) -> fmt :: Result) -> impl fmt :: Display { struct Printer < F > { f : F , } impl < F > fmt :: Display for Printer < F > where F : Fn (& mut fmt :: Formatter < '_ >) -> fmt :: Result , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (self . f) (fmt) } } Printer { f } }
};
}
