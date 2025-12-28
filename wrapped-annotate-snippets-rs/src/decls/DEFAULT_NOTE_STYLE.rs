macro_rules! DEFAULT_NOTE_STYLE {
    () => {
        # [doc = " [`Renderer::note`] applied by [`Renderer::styled`]"] pub const DEFAULT_NOTE_STYLE : Style = AnsiColor :: BrightGreen . on_default () . effects (Effects :: BOLD) ;
    };
}

DEFAULT_NOTE_STYLE!()