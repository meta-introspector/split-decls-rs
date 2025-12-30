// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl Drop for AnimationFrame { fn drop (& mut self) { if self . callback_wrapper . borrow_mut () . is_some () { web_sys :: window () . unwrap_throw () . cancel_animation_frame (self . render_id) . unwrap_throw () } } }
};
}
