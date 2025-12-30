// Generated macro for impl_from_unsigned (macro)
macro_rules! Depcrate_numberimpl_from_unsigned {
() => {
// Module: crate::number
// Provides: {"impl_from_unsigned"}
// Dependencies: {}
macro_rules ! impl_from_unsigned { ($ ($ ty : ty) ,*) => { $ (impl From <$ ty > for Number { fn from (u : $ ty) -> Self { let n = { # [cfg (not (feature = "arbitrary_precision"))] { N :: PosInt (u as u64) } # [cfg (feature = "arbitrary_precision")] { itoa :: Buffer :: new () . format (u) . to_owned () } } ; Number { n } } }) * } ; }
};
}
