// Generated macro for test_while_readonly (function)
macro_rules! Depcrate_scoped_runtest_while_readonly {
() => {
// Module: crate::scoped_run
// Provides: {"test_while_readonly"}
// Dependencies: {}
# [doc = " Ensure that the path P is read-only while the test runs, and restore original permissions at the"] # [doc = " end so compiletest can clean up. This will panic on Windows if the path is a directory (as it"] # [doc = " would otherwise do nothing)"] # [doc = ""] # [doc = " # Pitfalls"] # [doc = ""] # [doc = " - Some CI runners are ran as root which may bypass read-only permission restrictions. Unclear"] # [doc = "   exactly when such scenarios occur."] # [doc = ""] # [doc = " # FIXME"] # [doc = ""] # [doc = " FIXME(Oneirical): This will no longer be required after compiletest receives the ability to"] # [doc = " manipulate read-only files. See <https://github.com/rust-lang/rust/issues/126334>."] # [track_caller] pub fn test_while_readonly < P , F > (path : P , closure : F) where P : AsRef < Path > , F : FnOnce () + std :: panic :: UnwindSafe , { let path = path . as_ref () ; if is_windows () && path . is_dir () { eprintln ! ("This helper function cannot be used on Windows to make directories readonly.") ; eprintln ! ("See the official documentation:
            https://doc.rust-lang.org/std/fs/struct.Permissions.html#method.set_readonly") ; panic ! ("`test_while_readonly` on directory detected while on Windows.") ; } let metadata = fs :: metadata (& path) ; let original_perms = metadata . permissions () ; let mut new_perms = original_perms . clone () ; new_perms . set_readonly (true) ; fs :: set_permissions (& path , new_perms) ; let success = std :: panic :: catch_unwind (closure) ; fs :: set_permissions (& path , original_perms) ; success . unwrap () ; }
};
}
