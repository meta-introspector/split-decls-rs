// Generated macro for Options (struct)
macro_rules! Depcrate_entry_statOptions {
() => {
// Module: crate::entry::stat
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Configuration for comparing stat entries"] # [derive (Debug , PartialEq , Eq , Hash , Copy , Clone)] pub struct Options { # [doc = " If true, a files creation time is taken into consideration when checking if a file changed."] # [doc = " Can be set to false in case other tools alter the creation time in ways that interfere with our operation."] # [doc = ""] # [doc = " Default `true`."] pub trust_ctime : bool , # [doc = " If true, all stat fields will be used when checking for up-to-date'ness of the entry. Otherwise"] # [doc = " nano-second parts of mtime and ctime,uid, gid, inode and device number _will not_ be used, leaving only"] # [doc = " the whole-second part of ctime and mtime and the file size to be checked."] # [doc = ""] # [doc = " Default `true`."] pub check_stat : bool , # [doc = " Whether to compare nano secs when comparing timestamps. This currently"] # [doc = " leads to many false positives on linux and is therefore disabled there."] # [doc = ""] # [doc = " Default `false`"] pub use_nsec : bool , # [doc = " Whether to compare network devices secs when comparing timestamps."] # [doc = " Disabled by default because this can cause many false positives on network"] # [doc = " devices where the device number is not stable"] # [doc = ""] # [doc = " Default `false`."] pub use_stdev : bool , }
};
}
