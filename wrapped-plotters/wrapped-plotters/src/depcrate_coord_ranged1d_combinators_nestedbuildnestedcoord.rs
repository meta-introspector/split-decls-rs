// Generated macro for BuildNestedCoord (trait)
macro_rules! Depcrate_coord_ranged1d_combinators_nestedBuildNestedCoord {
() => {
// Module: crate::coord::ranged1d::combinators::nested
// Provides: {"BuildNestedCoord"}
// Dependencies: {}
# [doc = " Used to build a nested coordinate system."] pub trait BuildNestedCoord : AsRangedCoord where Self :: CoordDescType : DiscreteRanged , { # [doc = " Builds a nested coordinate system."] fn nested_coord < S : AsRangedCoord > (self , builder : impl Fn (< Self :: CoordDescType as Ranged > :: ValueType) -> S ,) -> NestedRange < Self :: CoordDescType , S :: CoordDescType > { let primary : Self :: CoordDescType = self . into () ; assert ! (primary . size () > 0) ; let secondary = primary . values () . map (| value | builder (value) . into ()) . collect () ; NestedRange { primary , secondary } } }
};
}
