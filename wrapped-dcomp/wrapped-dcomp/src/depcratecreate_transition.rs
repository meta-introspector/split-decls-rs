// Generated macro for create_transition (function)
macro_rules! Depcratecreate_transition {
() => {
// Module: crate
// Provides: {"create_transition"}
// Dependencies: {}
fn create_transition (library : & IUIAnimationTransitionLibrary2 , duration : f64 , final_value : f64 ,) -> Result < IUIAnimationTransition2 > { unsafe { library . CreateAccelerateDecelerateTransition (duration , final_value , 0.2 , 0.8) } }
};
}
