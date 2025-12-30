// Generated macro for Direction (trait)
macro_rules! Depcrate_series_surfaceDirection {
() => {
// Module: crate::series::surface
// Provides: {"Direction"}
// Dependencies: {}
# [doc = " Any type that describe a surface orientation"] pub trait Direction < X , Y , Z > { # [doc = " The type for the first input argument"] type Input1Type ; # [doc = " The type for the second input argument"] type Input2Type ; # [doc = " The output of the surface function"] type OutputType ; # [doc = " The function that maps a point on surface into the coordinate system"] fn make_coord (free_vars : (Self :: Input1Type , Self :: Input2Type) , result : Self :: OutputType ,) -> (X , Y , Z) ; }
};
}
