macro_rules! DEFAULT_CONTEXT_STYLE {
    () => {
        # [doc = " [`Renderer::context`] applied by [`Renderer::styled`]"] pub const DEFAULT_CONTEXT_STYLE : Style = BRIGHT_BLUE . effects (Effects :: BOLD) ;
    };
}

DEFAULT_CONTEXT_STYLE!()