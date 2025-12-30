// Generated macro for impl_1030 (impl)
macro_rules! Depcrate_config_tree_sections_statusimpl_1030 {
() => {
// Module: crate::config::tree::sections::status
// Provides: {"impl_1030"}
// Dependencies: {}
impl Status { # [doc = " The `status.showUntrackedFiles` key"] pub const SHOW_UNTRACKED_FILES : ShowUntrackedFiles = ShowUntrackedFiles :: new_with_validate ("showUntrackedFiles" , & config :: Tree :: STATUS , validate :: ShowUntrackedFiles ,) ; # [doc = " The `status.renameLimit` key."] pub const RENAME_LIMIT : keys :: UnsignedInteger = keys :: UnsignedInteger :: new_unsigned_integer ("renameLimit" , & config :: Tree :: MERGE ,) . with_note ("The limit is actually squared, so 1000 stands for up to 1 million diffs if fuzzy rename tracking is enabled" ,) ; # [doc = " The `status.renames` key."] pub const RENAMES : super :: diff :: Renames = super :: diff :: Renames :: new_renames ("renames" , & config :: Tree :: MERGE) ; }
};
}
