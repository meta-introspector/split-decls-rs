// Generated macro for impl_427 (impl)
macro_rules! Depcrate_observationimpl_427 {
() => {
// Module: crate::observation
// Provides: {"impl_427"}
// Dependencies: {}
impl VNRecognizedText { extern_methods ! (# [doc = " Calculate the bounding box around the characters in the range of"] # [doc = " the string."] # [doc = ""] # [doc = " The bounding boxes are not guaranteed to be an exact fit around"] # [doc = " the characters and are purely meant for UI purposes and not for"] # [doc = " image processing."] # [unsafe (method (boundingBoxForRange : error : _))] # [unsafe (method_family = none)] pub unsafe fn boundingBoxForRange_error (& self , range : NSRange ,) -> Result < Retained < VNRectangleObservation >, Retained < NSError >>;) ; }
};
}
