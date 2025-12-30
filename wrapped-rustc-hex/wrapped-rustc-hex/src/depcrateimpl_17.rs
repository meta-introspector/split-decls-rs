// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
# [cfg (not (feature = "std"))] impl fmt :: Display for FromHexError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { InvalidHexCharacter (ch , idx) => { f . write_str ("invalid character: ") ? ; ch . fmt (f) ? ; f . write_str (" at index ") ? ; idx . fmt (f) } InvalidHexLength => f . write_str ("invalid length") , } } }
};
}
