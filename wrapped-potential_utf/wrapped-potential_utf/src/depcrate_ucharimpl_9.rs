// Generated macro for impl_9 (impl)
macro_rules! Depcrate_ucharimpl_9 {
() => {
// Module: crate::uchar
// Provides: {"impl_9"}
// Dependencies: {}
impl fmt :: Debug for PotentialCodePoint { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . try_to_char () { Ok (c) => fmt :: Debug :: fmt (& c , f) , Err (_) => fmt :: Debug :: fmt (& self . 0 , f) , } } }
};
}
