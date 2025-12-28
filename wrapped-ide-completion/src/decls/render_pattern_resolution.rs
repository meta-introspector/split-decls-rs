macro_rules! deps {
    () => {
        PatternContext!();
        RenderContext!();
        Builder!();
    };
}

macro_rules! render_pattern_resolution {
    () => {
        deps!();
        pub (crate) fn render_pattern_resolution (ctx : RenderContext < '_ > , pattern_ctx : & PatternContext , local_name : hir :: Name , resolution : ScopeDef ,) -> Builder { render_resolution_pat (ctx , pattern_ctx , local_name , None , resolution) }
    };
}

render_pattern_resolution!();