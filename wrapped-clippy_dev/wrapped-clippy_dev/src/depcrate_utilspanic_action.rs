// Generated macro for panic_action (function)
macro_rules! Depcrate_utilspanic_action {
() => {
// Module: crate::utils
// Provides: {"panic_action"}
// Dependencies: {}
# [cold] # [track_caller] pub fn panic_action (err : & impl Display , action : ErrAction , path : & Path) -> ! { panic ! ("error {} `{}`: {}" , action . as_str () , path . display () , * err) }
};
}
