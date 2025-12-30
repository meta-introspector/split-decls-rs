// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl < TLeft , TRight > Display for StrComparison < '_ , TLeft , TRight > where TLeft : AsRef < str > + ? Sized , TRight : AsRef < str > + ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { printer :: write_header (f) ? ; printer :: write_lines (f , self . left . as_ref () , self . right . as_ref ()) } }
};
}
