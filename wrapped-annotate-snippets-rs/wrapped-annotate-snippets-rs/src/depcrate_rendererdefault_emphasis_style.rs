// Generated macro for DEFAULT_EMPHASIS_STYLE (const)
macro_rules! Depcrate_rendererDEFAULT_EMPHASIS_STYLE {
() => {
// Module: crate::renderer
// Provides: {"DEFAULT_EMPHASIS_STYLE"}
// Dependencies: {}
# [doc = " [`Renderer::emphasis`] applied by [`Renderer::styled`]"] pub const DEFAULT_EMPHASIS_STYLE : Style = if USE_WINDOWS_COLORS { AnsiColor :: BrightWhite . on_default () } else { Style :: new () } . effects (Effects :: BOLD) ;
};
}
