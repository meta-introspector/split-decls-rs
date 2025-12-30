// Generated macro for add_hide_transition (function)
macro_rules! Depcrateadd_hide_transition {
() => {
// Module: crate
// Provides: {"add_hide_transition"}
// Dependencies: {}
fn add_hide_transition (library : & IUIAnimationTransitionLibrary2 , storyboard : & IUIAnimationStoryboard2 , key_frame : UI_ANIMATION_KEYFRAME , final_value : f64 , card : & Card ,) -> Result < () > { unsafe { let transition = create_transition (library , 1.0 , final_value) ? ; storyboard . AddTransitionAtKeyframe (& card . variable , & transition , key_frame) } }
};
}
