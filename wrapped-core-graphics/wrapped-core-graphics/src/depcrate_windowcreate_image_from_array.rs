// Generated macro for create_image_from_array (function)
macro_rules! Depcrate_windowcreate_image_from_array {
() => {
// Module: crate::window
// Provides: {"create_image_from_array"}
// Dependencies: {}
pub fn create_image_from_array (screen_bounds : CGRect , window_array : CFArray , image_option : CGWindowImageOption ,) -> Option < CGImage > { unsafe { let image = CGWindowListCreateImageFromArray (screen_bounds , window_array . as_concrete_TypeRef () , image_option ,) ; if image . is_null () { None } else { Some (CGImage :: from_ptr (image)) } } }
};
}
