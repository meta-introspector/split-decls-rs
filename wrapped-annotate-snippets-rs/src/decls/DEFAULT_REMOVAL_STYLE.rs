macro_rules! DEFAULT_REMOVAL_STYLE {
    () => {
        # [doc = " [`Renderer::removal`] applied by [`Renderer::styled`]"] pub const DEFAULT_REMOVAL_STYLE : Style = AnsiColor :: BrightRed . on_default () ;
    };
}

DEFAULT_REMOVAL_STYLE!()