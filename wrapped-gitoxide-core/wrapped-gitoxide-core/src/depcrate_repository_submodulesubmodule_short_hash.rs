// Generated macro for submodule_short_hash (function)
macro_rules! Depcrate_repository_submodulesubmodule_short_hash {
() => {
// Module: crate::repository::submodule
// Provides: {"submodule_short_hash"}
// Dependencies: {}
fn submodule_short_hash (id : Option < gix :: ObjectId > , repo : Option < & Repository >) -> String { id . map_or_else (| | "none" . to_string () , | id | repo . map_or_else (| | id . to_string () , | repo | id . attach (repo) . shorten_or_id () . to_string ()) ,) }
};
}
