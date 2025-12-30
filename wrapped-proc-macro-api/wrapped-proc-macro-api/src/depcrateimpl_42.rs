// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
impl fmt :: Display for ServerError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . message . fmt (f) ? ; if let Some (io) = & self . io { f . write_str (": ") ? ; io . fmt (f) ? ; } Ok (()) } }
};
}
