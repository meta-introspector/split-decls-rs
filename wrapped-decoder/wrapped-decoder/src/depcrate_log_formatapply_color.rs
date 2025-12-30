// Generated macro for apply_color (function)
macro_rules! Depcrate_log_formatapply_color {
() => {
// Module: crate::log::format
// Provides: {"apply_color"}
// Dependencies: {}
fn apply_color (s : ColoredString , log_color : Option < LogColor > , level : Option < Level > ,) -> ColoredString { match log_color { Some (color) => match color { LogColor :: Color (c) => s . color (c) , LogColor :: SeverityLevel => match level { Some (level) => s . color (color_for_log_level (level)) , None => s , } , LogColor :: WarnError => match level { Some (level @ (Level :: Warn | Level :: Error)) => s . color (color_for_log_level (level)) , _ => s , } , } , None => s , } }
};
}
