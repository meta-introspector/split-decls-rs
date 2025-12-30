// Generated macro for CGDisplayReconfigurationCallBack (type)
macro_rules! Depcrate_displayCGDisplayReconfigurationCallBack {
() => {
// Module: crate::display
// Provides: {"CGDisplayReconfigurationCallBack"}
// Dependencies: {}
# [doc = " A client-supplied callback function that’s invoked whenever the configuration of a local display is changed."] pub type CGDisplayReconfigurationCallBack = unsafe extern "C" fn (display : CGDirectDisplayID , flags : u32 , user_info : * const c_void) ;
};
}
