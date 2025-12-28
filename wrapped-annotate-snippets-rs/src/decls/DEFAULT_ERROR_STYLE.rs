macro_rules! DEFAULT_ERROR_STYLE {
    () => {
        # [doc = " [`Renderer::error`] applied by [`Renderer::styled`]"] pub const DEFAULT_ERROR_STYLE : Style = AnsiColor :: BrightRed . on_default () . effects (Effects :: BOLD) ;
    };
}

DEFAULT_ERROR_STYLE!()