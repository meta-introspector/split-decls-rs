// Generated macro for IntoLinspace (trait)
macro_rules! Depcrate_coord_ranged1d_combinators_linspaceIntoLinspace {
() => {
// Module: crate::coord::ranged1d::combinators::linspace
// Provides: {"IntoLinspace"}
// Dependencies: {}
# [doc = " Makes a linspace coordinate from the ranged coordinates."] pub trait IntoLinspace : AsRangedCoord { # [doc = " Set the step value, make a linspace coordinate from the given range."] # [doc = " By default the matching method use the exact match"] # [doc = ""] # [doc = " - `val`: The step value"] # [doc = " - **returns*: The newly created linspace"] fn step < S : Clone > (self , val : S) -> Linspace < Self :: CoordDescType , S , Exact < Self :: Value > > where Self :: Value : Add < S , Output = Self :: Value > + PartialOrd + Clone , { let mut ret = Linspace { step : val , inner : self . into () , grid_value : vec ! [] , _phatom : PhantomData , } ; ret . compute_grid_values () ; ret } }
};
}
