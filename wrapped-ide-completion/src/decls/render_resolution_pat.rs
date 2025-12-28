macro_rules! deps {
    () => {
        RenderContext!();
        PatternContext!();
        Builder!();
    };
}

macro_rules! render_resolution_pat {
    () => {
        deps!();
        fn render_resolution_pat (ctx : RenderContext < '_ > , pattern_ctx : & PatternContext , local_name : hir :: Name , import_to_add : Option < LocatedImport > , resolution : ScopeDef ,) -> Builder { let _p = tracing :: info_span ! ("render_resolution_pat") . entered () ; use hir :: ModuleDef :: * ; if let ScopeDef :: ModuleDef (Macro (mac)) = resolution { let ctx = ctx . import_to_add (import_to_add) ; render_macro_pat (ctx , pattern_ctx , local_name , mac) } else { render_resolution_simple_ (ctx , & local_name , import_to_add , resolution) } }
    };
}

render_resolution_pat!()