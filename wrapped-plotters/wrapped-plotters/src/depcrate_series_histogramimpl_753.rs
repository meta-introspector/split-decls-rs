// Generated macro for impl_753 (impl)
macro_rules! Depcrate_series_histogramimpl_753 {
() => {
// Module: crate::series::histogram
// Provides: {"impl_753"}
// Dependencies: {}
impl < 'a , BR , A > Histogram < 'a , BR , A , Horizontal > where BR : DiscreteRanged + Clone , A : AddAssign < A > + Default + 'a , { # [doc = "\n    Creates a horizontal histogram.\n\n    See [`Histogram`] for more information and examples.\n    "] pub fn horizontal < ACoord , DB : DrawingBackend > (parent : & ChartContext < DB , Cartesian2d < ACoord , BR > > ,) -> Self where ACoord : Ranged < ValueType = A > , { let dp = parent . as_coord_spec () . y_spec () ; Self :: empty (dp) } }
};
}
