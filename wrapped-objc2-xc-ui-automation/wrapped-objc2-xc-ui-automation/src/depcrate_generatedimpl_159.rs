// Generated macro for impl_159 (impl)
macro_rules! Depcrate_generatedimpl_159 {
() => {
// Module: crate::generated
// Provides: {"impl_159"}
// Dependencies: {}
impl XCUIScreen { extern_methods ! (# [unsafe (method (new))] # [unsafe (method_family = new)] pub fn new (mtm : MainThreadMarker) -> Retained < Self >; # [unsafe (method (init))] # [unsafe (method_family = init)] pub fn init (this : Allocated < Self >) -> Retained < Self >; # [doc = " Returns the current device's main screen."] # [unsafe (method (mainScreen))] # [unsafe (method_family = none)] pub fn mainScreen (mtm : MainThreadMarker) -> Retained < XCUIScreen >; # [doc = " Returns the list of active screens."] # [doc = " The first screen returned in the list is the main screen."] # [unsafe (method (screens))] # [unsafe (method_family = none)] pub fn screens (mtm : MainThreadMarker) -> Retained < NSArray < XCUIScreen >>;) ; }
};
}
