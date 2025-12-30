// Generated macro for integrity (function)
macro_rules! Depcrate_repository_verifyintegrity {
() => {
// Module: crate::repository::verify
// Provides: {"integrity"}
// Dependencies: {}
pub fn integrity (repo : gix :: Repository , mut out : impl std :: io :: Write , mut progress : impl gix :: NestedProgress + 'static , should_interrupt : & AtomicBool , Context { output_statistics , thread_limit , verify_mode , algorithm , } : Context ,) -> anyhow :: Result < () > { # [cfg_attr (not (feature = "serde") , allow (unused))] let outcome = repo . objects . store_ref () . verify_integrity (& mut progress , should_interrupt , gix :: odb :: pack :: index :: verify :: integrity :: Options { verify_mode , traversal : algorithm . into () , thread_limit , make_pack_lookup_cache : | | gix :: odb :: pack :: cache :: Never , } ,) ? ; if let Some (index) = repo . worktree () . map (| wt | wt . index ()) . transpose () ? { index . verify_integrity () ? ; index . verify_entries () ? ; index . verify_extensions (true , repo . objects) ? ; progress . info (format ! ("Index at '{}' OK" , index . path () . display ())) ; } match output_statistics { Some (OutputFormat :: Human) => writeln ! (out , "Human output is currently unsupported, use JSON instead") ? , # [cfg (feature = "serde")] Some (OutputFormat :: Json) => { serde_json :: to_writer_pretty (out , & serde_json :: json ! ({ "index_statistics" : outcome . index_statistics , "loose_object-stores" : outcome . loose_object_stores }) ,) ? ; } None => { } } Ok (()) }
};
}
