// Generated macro for AnimationFrame (struct)
macro_rules! DepcrateAnimationFrame {
() => {
// Module: crate
// Provides: {"AnimationFrame"}
// Dependencies: {}
# [doc = " Handle for [`request_animation_frame`]."] # [derive (Debug)] pub struct AnimationFrame { render_id : i32 , _closure : Closure < dyn Fn (JsValue) > , callback_wrapper : Rc < RefCell < Option < CallbackWrapper > > > , }
};
}
