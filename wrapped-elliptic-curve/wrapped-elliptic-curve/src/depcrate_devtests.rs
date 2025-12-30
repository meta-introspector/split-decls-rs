// Generated macro for tests (module)
macro_rules! Depcrate_devtests {
() => {
// Module: crate::dev
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Scalar ; use ff :: PrimeField ; use hex_literal :: hex ; # [test] fn round_trip () { let bytes = hex ! ("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721") ; let scalar = Scalar :: from_repr (bytes . into ()) . unwrap () ; assert_eq ! (& bytes , scalar . to_repr () . as_slice ()) ; } }
};
}
