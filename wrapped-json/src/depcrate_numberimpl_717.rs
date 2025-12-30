// Generated macro for impl_717 (impl)
macro_rules! Depcrate_numberimpl_717 {
() => {
// Module: crate::number
// Provides: {"impl_717"}
// Dependencies: {}
impl Display for Number { # [cfg (not (feature = "arbitrary_precision"))] fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { match self . n { N :: PosInt (u) => formatter . write_str (itoa :: Buffer :: new () . format (u)) , N :: NegInt (i) => formatter . write_str (itoa :: Buffer :: new () . format (i)) , N :: Float (f) => formatter . write_str (ryu :: Buffer :: new () . format_finite (f)) , } } # [cfg (feature = "arbitrary_precision")] fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { Display :: fmt (& self . n , formatter) } }
};
}
