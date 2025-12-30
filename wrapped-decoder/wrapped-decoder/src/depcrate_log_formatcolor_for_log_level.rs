// Generated macro for color_for_log_level (function)
macro_rules! Depcrate_log_formatcolor_for_log_level {
() => {
// Module: crate::log::format
// Provides: {"color_for_log_level"}
// Dependencies: {}
fn color_for_log_level (level : Level) -> Color { match level { Level :: Error => Color :: Red , Level :: Warn => Color :: Yellow , Level :: Info => Color :: Green , Level :: Debug => Color :: BrightWhite , Level :: Trace => Color :: BrightBlack , } }
};
}
