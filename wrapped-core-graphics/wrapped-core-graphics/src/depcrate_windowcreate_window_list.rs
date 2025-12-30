// Generated macro for create_window_list (function)
macro_rules! Depcrate_windowcreate_window_list {
() => {
// Module: crate::window
// Provides: {"create_window_list"}
// Dependencies: {}
pub fn create_window_list (option : CGWindowListOption , relative_to_window : CGWindowID ,) -> Option < CFArray < CGWindowID > > { unsafe { let array = CGWindowListCreate (option , relative_to_window) ; if array . is_null () { None } else { Some (TCFType :: wrap_under_create_rule (array)) } } }
};
}
