// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl Algorithm { # [doc = " Create an instance of a negotiator which implements this algorithm."] pub fn into_negotiator (self) -> Box < dyn Negotiator > { match & self { Algorithm :: Noop => Box :: new (noop :: Noop) as Box < dyn Negotiator > , Algorithm :: Consecutive => Box :: < consecutive :: Algorithm > :: default () , Algorithm :: Skipping => Box :: < skipping :: Algorithm > :: default () , } } }
};
}
