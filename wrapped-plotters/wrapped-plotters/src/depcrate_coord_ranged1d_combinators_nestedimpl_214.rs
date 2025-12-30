// Generated macro for impl_214 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_nestedimpl_214 {
() => {
// Module: crate::coord::ranged1d::combinators::nested
// Provides: {"impl_214"}
// Dependencies: {}
impl < C , V > NestedValue < C , V > { # [doc = " Get the category of current nest value"] pub fn category (& self) -> & C { match self { NestedValue :: Category (cat) => cat , NestedValue :: Value (cat , _) => cat , } } # [doc = " Get the nested value from this value"] pub fn nested_value (& self) -> Option < & V > { match self { NestedValue :: Category (_) => None , NestedValue :: Value (_ , val) => Some (val) , } } }
};
}
