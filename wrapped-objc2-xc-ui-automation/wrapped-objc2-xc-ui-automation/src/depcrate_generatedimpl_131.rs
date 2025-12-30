// Generated macro for impl_131 (impl)
macro_rules! Depcrate_generatedimpl_131 {
() => {
// Module: crate::generated
// Provides: {"impl_131"}
// Dependencies: {}
impl XCUICoordinate { extern_methods ! (# [unsafe (method (new))] # [unsafe (method_family = new)] pub fn new (mtm : MainThreadMarker) -> Retained < Self >; # [unsafe (method (init))] # [unsafe (method_family = init)] pub fn init (this : Allocated < Self >) -> Retained < Self >; # [doc = " The element that the coordinate is based on, either directly or via the coordinate from which it was derived."] # [unsafe (method (referencedElement))] # [unsafe (method_family = none)] pub fn referencedElement (& self) -> Retained < XCUIElement >; # [cfg (feature = "objc2-core-foundation")] # [doc = " The dynamically computed value of the coordinate's location on screen. Note that this value is dependent on the current frame of the referenced element; if the element's frame changes, so will the value returned by this property. If the referenced element does exist when this is called, it will fail the test; check the referenced element's exists property if the element may not be present."] # [unsafe (method (screenPoint))] # [unsafe (method_family = none)] pub fn screenPoint (& self) -> CGPoint ; # [cfg (feature = "objc2-core-foundation")] # [doc = " Creates a new coordinate with an absolute offset in points from the original coordinate."] # [unsafe (method (coordinateWithOffset :))] # [unsafe (method_family = none)] pub fn coordinateWithOffset (& self , offset_vector : CGVector) -> Retained < XCUICoordinate >;) ; }
};
}
