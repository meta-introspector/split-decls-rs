// Generated macro for request_animation_frame (function)
macro_rules! Depcraterequest_animation_frame {
() => {
// Module: crate
// Provides: {"request_animation_frame"}
// Dependencies: {}
fn request_animation_frame (f : & Closure < dyn FnMut () >) { window () . request_animation_frame (f . as_ref () . unchecked_ref ()) . expect ("should register `requestAnimationFrame` OK") ; }
};
}
