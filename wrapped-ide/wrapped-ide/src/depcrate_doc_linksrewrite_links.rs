// Generated macro for rewrite_links (function)
macro_rules! Depcrate_doc_linksrewrite_links {
() => {
// Module: crate::doc_links
// Provides: {"rewrite_links"}
// Dependencies: {}
# [doc = " Rewrite documentation links in markdown to point to an online host (e.g. docs.rs)"] pub (crate) fn rewrite_links (db : & RootDatabase , markdown : & str , definition : Definition , range_map : Option < DocsRangeMap > ,) -> String { let mut cb = broken_link_clone_cb ; let doc = Parser :: new_with_broken_link_callback (markdown , MARKDOWN_OPTIONS , Some (& mut cb)) . into_offset_iter () ; let doc = map_links (doc , | target , title , range , link_type | { if target . contains ("://") { (Some (LinkType :: Inline) , target . to_owned () , title . to_owned ()) } else { let text_range = TextRange :: new (range . start . try_into () . unwrap () , range . end . try_into () . unwrap ()) ; let is_inner_doc = range_map . as_ref () . and_then (| range_map | range_map . map (text_range)) . map (| (_ , attr_id) | attr_id . is_inner_attr ()) . unwrap_or (false) ; if let Some ((target , title)) = rewrite_intra_doc_link (db , definition , target , title , is_inner_doc , link_type) { (None , target , title) } else if let Some (target) = rewrite_url_link (db , definition , target) { (Some (LinkType :: Inline) , target , title . to_owned ()) } else { (None , target . to_owned () , title . to_owned ()) } } }) ; let mut out = String :: new () ; cmark_resume_with_options (doc , & mut out , None , CMarkOptions { code_block_token_count : 3 , .. Default :: default () } ,) . ok () ; out }
};
}
