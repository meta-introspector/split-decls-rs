macro_rules! DEFAULT_EMPHASIS_STYLE {
    () => {
        # [doc = " [`Renderer::emphasis`] applied by [`Renderer::styled`]"] pub const DEFAULT_EMPHASIS_STYLE : Style = if USE_WINDOWS_COLORS { AnsiColor :: BrightWhite . on_default () } else { Style :: new () } . effects (Effects :: BOLD) ;
    };
}

DEFAULT_EMPHASIS_STYLE!()