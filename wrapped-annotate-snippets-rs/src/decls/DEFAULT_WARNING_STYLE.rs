macro_rules! DEFAULT_WARNING_STYLE {
    () => {
        # [doc = " [`Renderer::warning`] applied by [`Renderer::styled`]"] pub const DEFAULT_WARNING_STYLE : Style = if USE_WINDOWS_COLORS { AnsiColor :: BrightYellow . on_default () } else { AnsiColor :: Yellow . on_default () } . effects (Effects :: BOLD) ;
    };
}

DEFAULT_WARNING_STYLE!();