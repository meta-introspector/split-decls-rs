// Generated macro for render_resolution_with_import_pat (function)
macro_rules! Depcrate_renderrender_resolution_with_import_pat {
() => {
// Module: crate::render
// Provides: {"render_resolution_with_import_pat"}
// Dependencies: {}
pub (crate) fn render_resolution_with_import_pat (ctx : RenderContext < '_ > , pattern_ctx : & PatternContext , import_edit : LocatedImport ,) -> Option < Builder > { let resolution = ScopeDef :: from (import_edit . original_item) ; let local_name = get_import_name (resolution , & ctx , & import_edit) ? ; Some (render_resolution_pat (ctx , pattern_ctx , local_name , Some (import_edit) , resolution)) }
};
}
