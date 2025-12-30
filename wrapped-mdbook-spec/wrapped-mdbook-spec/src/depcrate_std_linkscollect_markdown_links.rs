// Generated macro for collect_markdown_links (function)
macro_rules! Depcrate_std_linkscollect_markdown_links {
() => {
// Module: crate::std_links
// Provides: {"collect_markdown_links"}
// Dependencies: {}
# [doc = " Collects all markdown links that look like they might be standard library links."] fn collect_markdown_links < 'a > (chapter : & 'a Chapter , diag : & mut Diagnostics) -> Vec < Link < 'a > > { let mut opts = Options :: empty () ; opts . insert (Options :: ENABLE_TABLES) ; opts . insert (Options :: ENABLE_FOOTNOTES) ; opts . insert (Options :: ENABLE_STRIKETHROUGH) ; opts . insert (Options :: ENABLE_TASKLISTS) ; opts . insert (Options :: ENABLE_HEADING_ATTRIBUTES) ; opts . insert (Options :: ENABLE_SMART_PUNCTUATION) ; let mut broken_links = Vec :: new () ; let mut links = Vec :: new () ; let broken_link = | broken_link : BrokenLink < '_ > | { broken_links . push (Link { link_type : broken_link . link_type , dest_url : CowStr :: Boxed (broken_link . reference . into_string () . into ()) , range : broken_link . span . clone () , }) ; None } ; let parser = Parser :: new_with_broken_link_callback (& chapter . content , opts , Some (broken_link)) . into_offset_iter () ; for (event , range) in parser { match event { Event :: Start (Tag :: Link { link_type , dest_url , title , id : _ , }) => { if matches ! (link_type , LinkType :: Autolink | LinkType :: Email) { continue ; } if dest_url . starts_with ("http") || dest_url . contains (".md") || dest_url . contains (".html") || dest_url . starts_with ('#') { continue ; } if ! title . is_empty () { warn_or_err ! (diag , "titles in links are not supported\n\
                         Link {dest_url} has title `{title}` found in chapter {} ({:?})" , chapter . name , chapter . source_path . as_ref () . unwrap ()) ; } links . push (Link { link_type , dest_url , range : range . clone () , }) ; } _ => { } } } links . extend (broken_links) ; links }
};
}
