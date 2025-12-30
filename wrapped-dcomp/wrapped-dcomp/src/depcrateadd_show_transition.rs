// Generated macro for add_show_transition (function)
macro_rules! Depcrateadd_show_transition {
() => {
// Module: crate
// Provides: {"add_show_transition"}
// Dependencies: {}
fn add_show_transition (library : & IUIAnimationTransitionLibrary2 , storyboard : & IUIAnimationStoryboard2 , card : & Card ,) -> Result < UI_ANIMATION_KEYFRAME > { unsafe { let duration = (180.0 - card . variable . GetValue () ?) / 180.0 ; let transition = create_transition (library , duration , 180.0) ? ; storyboard . AddTransition (& card . variable , & transition) ? ; storyboard . AddKeyframeAfterTransition (& transition) } }
};
}
