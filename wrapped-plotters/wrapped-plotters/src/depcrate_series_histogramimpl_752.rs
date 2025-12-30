// Generated macro for impl_752 (impl)
macro_rules! Depcrate_series_histogramimpl_752 {
() => {
// Module: crate::series::histogram
// Provides: {"impl_752"}
// Dependencies: {}
impl < 'a , BR , A > Histogram < 'a , BR , A , Vertical > where BR : DiscreteRanged + Clone , A : AddAssign < A > + Default + 'a , { # [doc = "\n    Creates a vertical histogram.\n\n    See [`Histogram`] for more information and examples.\n    "] pub fn vertical < ACoord , DB : DrawingBackend + 'a > (parent : & ChartContext < DB , Cartesian2d < BR , ACoord > > ,) -> Self where ACoord : Ranged < ValueType = A > , { let dp = parent . as_coord_spec () . x_spec () ; Self :: empty (dp) } }
};
}
