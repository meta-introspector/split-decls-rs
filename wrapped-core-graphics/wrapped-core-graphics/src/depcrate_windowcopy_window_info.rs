// Generated macro for copy_window_info (function)
macro_rules! Depcrate_windowcopy_window_info {
() => {
// Module: crate::window
// Provides: {"copy_window_info"}
// Dependencies: {}
pub fn copy_window_info (option : CGWindowListOption , relative_to_window : CGWindowID ,) -> Option < CFArray > { unsafe { let array = CGWindowListCopyWindowInfo (option , relative_to_window) ; if array . is_null () { None } else { Some (TCFType :: wrap_under_create_rule (array)) } } }
};
}
