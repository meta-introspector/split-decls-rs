// Generated macro for DEFAULT_WARNING_STYLE (const)
macro_rules! Depcrate_rendererDEFAULT_WARNING_STYLE {
() => {
// Module: crate::renderer
// Provides: {"DEFAULT_WARNING_STYLE"}
// Dependencies: {}
# [doc = " [`Renderer::warning`] applied by [`Renderer::styled`]"] pub const DEFAULT_WARNING_STYLE : Style = if USE_WINDOWS_COLORS { AnsiColor :: BrightYellow . on_default () } else { AnsiColor :: Yellow . on_default () } . effects (Effects :: BOLD) ;
};
}
