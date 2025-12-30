// Generated macro for extract_definitions_from_docs (function)
macro_rules! Depcrate_doc_linksextract_definitions_from_docs {
() => {
// Module: crate::doc_links
// Provides: {"extract_definitions_from_docs"}
// Dependencies: {}
# [doc = " Extracts all links from a given markdown text returning the definition text range, link-text"] # [doc = " and the namespace if known."] pub (crate) fn extract_definitions_from_docs (docs : & Documentation ,) -> Vec < (TextRange , String , Option < hir :: Namespace >) > { Parser :: new_with_broken_link_callback (docs . as_str () , MARKDOWN_OPTIONS , Some (& mut broken_link_clone_cb) ,) . into_offset_iter () . filter_map (| (event , range) | match event { Event :: Start (Tag :: Link (_ , target , _)) => { let (link , ns) = parse_intra_doc_link (& target) ; Some ((TextRange :: new (range . start . try_into () . ok () ? , range . end . try_into () . ok () ?) , link . to_owned () , ns ,)) } _ => None , }) . collect () }
};
}
