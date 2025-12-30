// Generated macro for entries (function)
macro_rules! Depcrate_repository_mailmapentries {
() => {
// Module: crate::repository::mailmap
// Provides: {"entries"}
// Dependencies: {}
pub fn entries (repo : gix :: Repository , format : OutputFormat , # [cfg_attr (not (feature = "serde") , allow (unused_variables))] out : impl io :: Write , mut err : impl io :: Write ,) -> anyhow :: Result < () > { if format == OutputFormat :: Human { writeln ! (err , "Defaulting to JSON as human format isn't implemented") . ok () ; } let mut mailmap = gix :: mailmap :: Snapshot :: default () ; if let Err (e) = repo . open_mailmap_into (& mut mailmap) { writeln ! (err , "Error while loading mailmap, the first error is: {e}") . ok () ; } # [cfg (feature = "serde")] serde_json :: to_writer_pretty (out , & mailmap . iter () . map (JsonEntry :: from) . collect :: < Vec < _ > > ()) ? ; Ok (()) }
};
}
