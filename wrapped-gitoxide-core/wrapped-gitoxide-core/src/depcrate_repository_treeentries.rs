// Generated macro for entries (function)
macro_rules! Depcrate_repository_treeentries {
() => {
// Module: crate::repository::tree
// Provides: {"entries"}
// Dependencies: {}
pub fn entries (repo : gix :: Repository , treeish : Option < & str > , recursive : bool , extended : bool , format : OutputFormat , mut out : impl io :: Write ,) -> anyhow :: Result < () > { if format != OutputFormat :: Human { bail ! ("Only human output format is supported at the moment") ; } let tree = treeish_to_tree (treeish , & repo) ? ; if recursive { let mut write = BufWriter :: new (out) ; let mut delegate = entries :: Traverse :: new (extended . then_some (& repo) , Some (& mut write)) ; tree . traverse () . depthfirst (& mut delegate) ? ; } else { for entry in tree . iter () { let entry = entry ? ; format_entry (& mut out , & entry . inner , entry . inner . filename , extended . then (| | entry . id () . header () . map (| o | o . size ())) . transpose () ? ,) ? ; } } Ok (()) }
};
}
