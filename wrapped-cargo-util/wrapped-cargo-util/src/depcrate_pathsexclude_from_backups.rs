// Generated macro for exclude_from_backups (function)
macro_rules! Depcrate_pathsexclude_from_backups {
() => {
// Module: crate::paths
// Provides: {"exclude_from_backups"}
// Dependencies: {}
# [doc = " Marks the directory as excluded from archives/backups."] # [doc = ""] # [doc = " This is recommended to prevent derived/temporary files from bloating backups. There are two"] # [doc = " mechanisms used to achieve this right now:"] # [doc = ""] # [doc = " * A dedicated resource property excluding from Time Machine backups on macOS"] # [doc = " * CACHEDIR.TAG files supported by various tools in a platform-independent way"] fn exclude_from_backups (path : & Path) { exclude_from_time_machine (path) ; let file = path . join ("CACHEDIR.TAG") ; if ! file . exists () { let _ = std :: fs :: write (file , "Signature: 8a477f597d28d172789f06886806bc55
# This file is a cache directory tag created by cargo.
# For information about cache directory tags see https://bford.info/cachedir/
" ,) ; } }
};
}
