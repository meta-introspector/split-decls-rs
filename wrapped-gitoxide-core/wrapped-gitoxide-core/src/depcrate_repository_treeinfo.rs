// Generated macro for info (function)
macro_rules! Depcrate_repository_treeinfo {
() => {
// Module: crate::repository::tree
// Provides: {"info"}
// Dependencies: {}
# [cfg_attr (not (feature = "serde") , allow (unused_variables))] pub fn info (repo : gix :: Repository , treeish : Option < & str > , extended : bool , format : OutputFormat , out : impl io :: Write , mut err : impl io :: Write ,) -> anyhow :: Result < () > { if format == OutputFormat :: Human { writeln ! (err , "Only JSON is implemented - using that instead") ? ; } let tree = treeish_to_tree (treeish , & repo) ? ; let mut delegate = entries :: Traverse :: new (extended . then_some (& repo) , None) ; tree . traverse () . breadthfirst (& mut delegate) ? ; # [cfg (feature = "serde")] { delegate . stats . bytes = extended . then_some (delegate . stats . num_bytes) ; serde_json :: to_writer_pretty (out , & delegate . stats) ? ; } Ok (()) }
};
}
