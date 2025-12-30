// Generated macro for request_animation_frame (function)
macro_rules! Depcraterequest_animation_frame {
() => {
// Module: crate
// Provides: {"request_animation_frame"}
// Dependencies: {}
# [doc = " Calls browser's `requestAnimationFrame`. It is cancelled when the handler is dropped."] # [doc = ""] # [doc = " [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame)"] pub fn request_animation_frame < F > (callback_once : F) -> AnimationFrame where F : FnOnce (f64) + 'static , { let callback_wrapper = Rc :: new (RefCell :: new (Some (CallbackWrapper (Box :: new (callback_once))))) ; let callback : Closure < dyn Fn (JsValue) > = { let callback_wrapper = Rc :: clone (& callback_wrapper) ; Closure :: wrap (Box :: new (move | v : JsValue | { let time : f64 = v . as_f64 () . unwrap_or (0.0) ; let callback = callback_wrapper . borrow_mut () . take () . unwrap () . 0 ; callback (time) ; })) } ; let render_id = web_sys :: window () . unwrap_throw () . request_animation_frame (callback . as_ref () . unchecked_ref ()) . unwrap_throw () ; AnimationFrame { render_id , _closure : callback , callback_wrapper , } }
};
}
