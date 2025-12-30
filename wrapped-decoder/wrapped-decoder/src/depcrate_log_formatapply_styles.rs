// Generated macro for apply_styles (function)
macro_rules! Depcrate_log_formatapply_styles {
() => {
// Module: crate::log::format
// Provides: {"apply_styles"}
// Dependencies: {}
fn apply_styles (s : ColoredString , log_style : Option < & Vec < Styles > >) -> ColoredString { let Some (log_styles) = log_style else { return s ; } ; let mut stylized_string = s ; for style in log_styles { stylized_string = match style { Styles :: Bold => stylized_string . bold () , Styles :: Italic => stylized_string . italic () , Styles :: Underline => stylized_string . underline () , Styles :: Strikethrough => stylized_string . strikethrough () , Styles :: Dimmed => stylized_string . dimmed () , Styles :: Clear => stylized_string . clear () , Styles :: Reversed => stylized_string . reversed () , Styles :: Blink => stylized_string . blink () , Styles :: Hidden => stylized_string . hidden () , } ; } stylized_string }
};
}
