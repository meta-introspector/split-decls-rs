// Generated macro for create_transition (function)
macro_rules! Depcratecreate_transition {
() => {
// Module: crate
// Provides: {"create_transition"}
// Dependencies: {}
fn create_transition () -> Result < IUIAnimationTransition > { unsafe { let library : IUIAnimationTransitionLibrary = CoCreateInstance (& UIAnimationTransitionLibrary , None , CLSCTX_ALL) ? ; library . CreateAccelerateDecelerateTransition (5.0 , 1.0 , 0.2 , 0.8) } }
};
}
