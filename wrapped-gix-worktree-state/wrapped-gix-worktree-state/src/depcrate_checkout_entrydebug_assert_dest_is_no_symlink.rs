// Generated macro for debug_assert_dest_is_no_symlink (function)
macro_rules! Depcrate_checkout_entrydebug_assert_dest_is_no_symlink {
() => {
// Module: crate::checkout::entry
// Provides: {"debug_assert_dest_is_no_symlink"}
// Dependencies: {}
# [doc = " This is a debug assertion as we expect the machinery calling this to prevent this possibility in the first place"] # [cfg (debug_assertions)] fn debug_assert_dest_is_no_symlink (path : & Path) { if let Ok (meta) = path . metadata () { debug_assert ! (! meta . file_type () . is_symlink () , "BUG: should not ever allow to overwrite/write-into the target of a symbolic link: {}" , path . display ()) ; } }
};
}
