// Generated macro for impl_218 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_nestedimpl_218 {
() => {
// Module: crate::coord::ranged1d::combinators::nested
// Provides: {"impl_218"}
// Dependencies: {}
impl < PT , ST , P , S > ValueFormatter < NestedValue < PT , ST > > for NestedRange < P , S > where P : Ranged < ValueType = PT > + DiscreteRanged , S : Ranged < ValueType = ST > , P : ValueFormatter < PT > , S : ValueFormatter < ST > , { fn format (value : & NestedValue < PT , ST >) -> String { match value { NestedValue :: Category (cat) => P :: format (cat) , NestedValue :: Value (_ , val) => S :: format (val) , } } }
};
}
