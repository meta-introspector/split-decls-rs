// Generated macro for render_names (function)
macro_rules! Depcrate_grammarrender_names {
() => {
// Module: crate::grammar
// Provides: {"render_names"}
// Dependencies: {}
# [doc = " Helper to take a list of production names and to render all of those to a"] # [doc = " mixture of markdown and HTML."] fn render_names (grammar : & Grammar , names : & [& str] , link_map : & HashMap < String , String > , for_lexer : bool , chapter : & Chapter , diag : & mut Diagnostics ,) -> String { let for_summary = is_summary (chapter) ; let mut output = String :: new () ; output . push_str ("<div class=\"grammar-container\">\n\
         \n" ,) ; if for_lexer { output . push_str ("**<sup>Lexer</sup>**\n") ; } else { output . push_str ("**<sup>Syntax</sup>**\n") ; } output . push_str ("<br>\n") ; let update_link_map = | get_id : fn (& str , bool) -> String | -> HashMap < String , String > { link_map . iter () . map (| (name , path) | { let id = get_id (name , for_summary) ; let path = if for_summary { format ! ("#{id}") } else { format ! ("{path}#{id}") } ; (name . clone () , path) }) . collect () } ; let render_ctx = RenderCtx { md_link_map : update_link_map (render_markdown :: markdown_id) , rr_link_map : update_link_map (render_railroad :: railroad_id) , for_summary , } ; if let Err (e) = grammar . render_markdown (& render_ctx , & names , & mut output) { warn_or_err ! (diag , "grammar failed in chapter {:?}: {e}" , chapter . source_path . as_ref () . unwrap ()) ; } output . push_str ("\n\
         <button class=\"grammar-toggle-railroad\" type=\"button\" \
            title=\"Toggle railroad display\" \
            onclick=\"toggle_railroad()\">\
            Show Railroad\
         </button>\n\
         </div>\n\
         <div class=\"grammar-railroad grammar-hidden\">\n\
         \n" ,) ; if let Err (e) = grammar . render_railroad (& render_ctx , & names , & mut output) { warn_or_err ! (diag , "grammar failed in chapter {:?}: {e}" , chapter . source_path . as_ref () . unwrap ()) ; } output . push_str ("</div>\n") ; output }
};
}
