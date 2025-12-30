// Generated macro for ErrorBarOrient (trait)
macro_rules! Depcrate_element_errorbarErrorBarOrient {
() => {
// Module: crate::element::errorbar
// Provides: {"ErrorBarOrient"}
// Dependencies: {}
# [doc = "\nUsed to reuse code between horizontal and vertical error bars.\n\nThis is used internally by Plotters and should probably not be included in user code.\nSee [`ErrorBar`] for more information and examples.\n"] pub trait ErrorBarOrient < K , V > { type XType ; type YType ; fn make_coord (key : K , val : V) -> (Self :: XType , Self :: YType) ; fn ending_coord (coord : BackendCoord , w : u32) -> (BackendCoord , BackendCoord) ; }
};
}
