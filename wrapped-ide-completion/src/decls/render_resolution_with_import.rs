macro_rules! deps {
    () => {
        Builder!();
        PathCompletionCtx!();
        RenderContext!();
    };
}

macro_rules! render_resolution_with_import {
    () => {
        deps!();
        pub (crate) fn render_resolution_with_import (ctx : RenderContext < '_ > , path_ctx : & PathCompletionCtx < '_ > , import_edit : LocatedImport ,) -> Option < Builder > { let resolution = ScopeDef :: from (import_edit . original_item) ; let local_name = get_import_name (resolution , & ctx , & import_edit) ? ; let doc_aliases = ctx . completion . doc_aliases_in_scope (resolution) ; let ctx = ctx . doc_aliases (doc_aliases) ; Some (render_resolution_path (ctx , path_ctx , local_name , Some (import_edit) , resolution)) }
    };
}

render_resolution_with_import!()