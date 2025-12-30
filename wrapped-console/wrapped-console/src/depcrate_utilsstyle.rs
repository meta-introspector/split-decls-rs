// Generated macro for style (function)
macro_rules! Depcrate_utilsstyle {
() => {
// Module: crate::utils
// Provides: {"style"}
// Dependencies: {}
# [doc = " Wraps an object for formatting for styling."] # [doc = ""] # [doc = " Example:"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use console::style;"] # [doc = " format!(\"Hello {}\", style(\"World\").cyan());"] # [doc = " ```"] # [doc = ""] # [doc = " This is a shortcut for making a new style and applying it"] # [doc = " to a value:"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use console::Style;"] # [doc = " format!(\"Hello {}\", Style::new().cyan().apply_to(\"World\"));"] # [doc = " ```"] pub fn style < D > (val : D) -> StyledObject < D > { Style :: new () . apply_to (val) }
};
}
