macro_rules! deps {
    () => {
        Builder!();
        PathCompletionCtx!();
        RenderContext!();
    };
}

macro_rules! render_path_resolution {
    () => {
        deps!();
        pub (crate) fn render_path_resolution (ctx : RenderContext < '_ > , path_ctx : & PathCompletionCtx < '_ > , local_name : hir :: Name , resolution : ScopeDef ,) -> Builder { render_resolution_path (ctx , path_ctx , local_name , None , resolution) }
    };
}

render_path_resolution!();