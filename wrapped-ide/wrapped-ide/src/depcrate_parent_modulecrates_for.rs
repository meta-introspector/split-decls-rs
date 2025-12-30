// Generated macro for crates_for (function)
macro_rules! Depcrate_parent_modulecrates_for {
() => {
// Module: crate::parent_module
// Provides: {"crates_for"}
// Dependencies: {}
# [doc = " This returns `Vec` because a module may be included from several places."] pub (crate) fn crates_for (db : & RootDatabase , file_id : FileId) -> Vec < Crate > { db . relevant_crates (file_id) . iter () . copied () . filter (| & crate_id | { crate_def_map (db , crate_id) . modules_for_file (db , file_id) . next () . is_some () }) . sorted () . collect () }
};
}
