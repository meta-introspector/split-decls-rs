macro_rules! DEFAULT_HELP_STYLE {
    () => {
        # [doc = " [`Renderer::help`] applied by [`Renderer::styled`]"] pub const DEFAULT_HELP_STYLE : Style = AnsiColor :: BrightCyan . on_default () . effects (Effects :: BOLD) ;
    };
}

DEFAULT_HELP_STYLE!()