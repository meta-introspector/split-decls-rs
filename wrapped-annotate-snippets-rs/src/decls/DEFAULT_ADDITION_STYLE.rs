macro_rules! DEFAULT_ADDITION_STYLE {
    () => {
        # [doc = " [`Renderer::addition`] applied by [`Renderer::styled`]"] pub const DEFAULT_ADDITION_STYLE : Style = AnsiColor :: BrightGreen . on_default () ;
    };
}

DEFAULT_ADDITION_STYLE!()