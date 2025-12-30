// Generated macro for create_image (function)
macro_rules! Depcrate_windowcreate_image {
() => {
// Module: crate::window
// Provides: {"create_image"}
// Dependencies: {}
pub fn create_image (screen_bounds : CGRect , list_option : CGWindowListOption , window_id : CGWindowID , image_option : CGWindowImageOption ,) -> Option < CGImage > { unsafe { let image = CGWindowListCreateImage (screen_bounds , list_option , window_id , image_option) ; if image . is_null () { None } else { Some (CGImage :: from_ptr (image)) } } }
};
}
