macro_rules! DEFAULT_LINE_NUM_STYLE {
    () => {
        # [doc = " [`Renderer::line_num`] applied by [`Renderer::styled`]"] pub const DEFAULT_LINE_NUM_STYLE : Style = BRIGHT_BLUE . effects (Effects :: BOLD) ;
    };
}

DEFAULT_LINE_NUM_STYLE!();