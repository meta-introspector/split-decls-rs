// Generated macro for force_color_output (function)
macro_rules! Depcrate_styleforce_color_output {
() => {
// Module: crate::style
// Provides: {"force_color_output"}
// Dependencies: {}
# [doc = " Forces colored output on or off globally, overriding NO_COLOR."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " crossterm supports NO_COLOR (<https://no-color.org/>) to disabled colored output."] # [doc = ""] # [doc = " This API allows applications to override that behavior and force colorized output"] # [doc = " even if NO_COLOR is set."] pub fn force_color_output (enabled : bool) { Colored :: set_ansi_color_disabled (! enabled) }
};
}
