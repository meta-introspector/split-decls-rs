// Generated macro for arbitrary_param (function)
macro_rules! Depcrate_astarbitrary_param {
() => {
// Module: crate::ast
// Provides: {"arbitrary_param"}
// Dependencies: {}
# [doc = " Returns for a given type `ty` the associated item `Parameters` of the"] # [doc = " type's `Arbitrary` implementation."] pub fn arbitrary_param (ty : & syn :: Type) -> syn :: Type { parse_quote ! (<# ty as _proptest :: arbitrary :: Arbitrary >:: Parameters) }
};
}
