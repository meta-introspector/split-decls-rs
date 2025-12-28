macro_rules! deps {
    () => {
        PatternContext!();
        Builder!();
        RenderContext!();
    };
}

macro_rules! render_resolution_with_import_pat {
    () => {
        deps!();
        pub (crate) fn render_resolution_with_import_pat (ctx : RenderContext < '_ > , pattern_ctx : & PatternContext , import_edit : LocatedImport ,) -> Option < Builder > { let resolution = ScopeDef :: from (import_edit . original_item) ; let local_name = get_import_name (resolution , & ctx , & import_edit) ? ; Some (render_resolution_pat (ctx , pattern_ctx , local_name , Some (import_edit) , resolution)) }
    };
}

render_resolution_with_import_pat!();