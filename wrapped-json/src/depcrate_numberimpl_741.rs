// Generated macro for impl_741 (impl)
macro_rules! Depcrate_numberimpl_741 {
() => {
// Module: crate::number
// Provides: {"impl_741"}
// Dependencies: {}
impl Number { # [cfg (not (feature = "arbitrary_precision"))] # [cold] pub (crate) fn unexpected (& self) -> Unexpected { match self . n { N :: PosInt (u) => Unexpected :: Unsigned (u) , N :: NegInt (i) => Unexpected :: Signed (i) , N :: Float (f) => Unexpected :: Float (f) , } } # [cfg (feature = "arbitrary_precision")] # [cold] pub (crate) fn unexpected (& self) -> Unexpected { Unexpected :: Other ("number") } }
};
}
