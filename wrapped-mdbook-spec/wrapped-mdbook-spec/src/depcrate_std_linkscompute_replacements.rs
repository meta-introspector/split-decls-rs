// Generated macro for compute_replacements (function)
macro_rules! Depcrate_std_linkscompute_replacements {
() => {
// Module: crate::std_links
// Provides: {"compute_replacements"}
// Dependencies: {}
# [doc = " Computes the replacements to make in the markdown content."] # [doc = ""] # [doc = " Returns a `Vec` of `(md_link, url, range)` where:"] # [doc = ""] # [doc = " - `md_link` is the markdown link string to show to the user (like `[foo]`)."] # [doc = " - `url` is the URL to the standard library."] # [doc = " - `range` is the range in the original markdown to replace with the new link."] fn compute_replacements < 'a > (chapter : & 'a Chapter , links : & [Link < '_ >] , urls : & [& 'a str] , diag : & mut Diagnostics ,) -> Vec < (& 'a str , & 'a str , Range < usize >) > { let mut replacements = Vec :: new () ; for (url , link) in urls . iter () . zip (links) { let Some (cap) = ANCHOR_URL . captures (url) else { let line = super :: line_from_range (& chapter . content , & link . range) ; warn_or_err ! (diag , "broken markdown link found in {}\n\
                Line is: {line}\n\
                Link to `{}` could not be resolved by rustdoc to a known URL (result was `{}`).\n" , chapter . source_path . as_ref () . unwrap () . display () , link . dest_url , url) ; continue ; } ; let url = cap . get (1) . unwrap () . as_str () ; let md_link = & chapter . content [link . range . clone ()] ; let range = link . range . clone () ; let add_link = | re : & Regex | { let Some (cap) = re . captures (md_link) else { bug ! ("expected link `{md_link}` of type {:?} to match regex {re}" , link . link_type) ; } ; let md_link = cap . get (1) . unwrap () . as_str () ; replacements . push ((md_link , url , range)) ; } ; match link . link_type { LinkType :: Inline => { add_link (& MD_LINK_INLINE) ; } LinkType :: Reference | LinkType :: Collapsed => { add_link (& MD_LINK_REFERENCE) ; } LinkType :: Shortcut => { add_link (& MD_LINK_SHORTCUT) ; } _ => { bug ! ("unexpected link type: {link:#?}") ; } } } replacements . sort_by (| a , b | b . 2 . clone () . partial_cmp (a . 2 . clone ()) . unwrap ()) ; replacements }
};
}
