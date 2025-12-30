// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl fmt :: Display for LamportsError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Self :: ArithmeticUnderflow => f . write_str ("Arithmetic underflowed") , Self :: ArithmeticOverflow => f . write_str ("Arithmetic overflowed") , } } }
};
}
