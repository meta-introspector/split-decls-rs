// Generated macro for impl_102 (impl)
macro_rules! Depcrate_generatedimpl_102 {
() => {
// Module: crate::generated
// Provides: {"impl_102"}
// Dependencies: {}
# [doc = " XCUIElementTypeSlider."] # [doc = ""] # [doc = " This category on XCUIElement provides functionality for automating UISlider and NSSlider."] impl XCUIElement { extern_methods ! (# [cfg (feature = "objc2-core-foundation")] # [doc = " Manipulates the UI to change the displayed value of the slider to one based on a normalized position. 0 corresponds to the minimum value of the slider, 1 corresponds to its maximum value. The adjustment is a \"best effort\" to move the indicator to the desired position; absolute fidelity is not guaranteed."] # [unsafe (method (adjustToNormalizedSliderPosition :))] # [unsafe (method_family = none)] pub fn adjustToNormalizedSliderPosition (& self , normalized_slider_position : CGFloat) ; # [cfg (feature = "objc2-core-foundation")] # [doc = " Returns the position of the slider's indicator as a normalized value where 0 corresponds to the minimum value of the slider and 1 corresponds to its maximum value."] # [unsafe (method (normalizedSliderPosition))] # [unsafe (method_family = none)] pub fn normalizedSliderPosition (& self) -> CGFloat ;) ; }
};
}
