// Generated macro for create_effects (function)
macro_rules! Depcrate_styled_strcreate_effects {
() => {
// Module: crate::styled_str
// Provides: {"create_effects"}
// Dependencies: {}
fn create_effects (category : & CategorisedSlice < '_ >) -> Effects { Effects :: new () . set (Effects :: ITALIC , category . italic . unwrap_or (false)) . set (Effects :: BLINK , category . blink . unwrap_or (false)) . set (Effects :: INVERT , category . reversed . unwrap_or (false)) . set (Effects :: HIDDEN , category . hidden . unwrap_or (false)) . set (Effects :: STRIKETHROUGH , category . strikethrough . unwrap_or (false) ,) . set (Effects :: UNDERLINE , category . underline . unwrap_or (false)) . set (Effects :: BOLD , is_bold (category . intensity)) . set (Effects :: DIMMED , is_faint (category . intensity)) }
};
}
