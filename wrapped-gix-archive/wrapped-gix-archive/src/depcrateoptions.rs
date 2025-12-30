// Generated macro for Options (struct)
macro_rules! DepcrateOptions {
() => {
// Module: crate
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options for configuring [`write_stream()`]."] # [derive (Clone , Debug)] pub struct Options { # [doc = " The archive's format."] pub format : Format , # [doc = " Given a `path`, originating in the git tree, to place into the archive, put `<prefix>/path` in front of it."] # [doc = ""] # [doc = " Note that `/` should be used as separator, and that a prefix directory has to end with `/`."] pub tree_prefix : Option < BString > , # [doc = " The modification time for all entries in the archive as seen since UNIX epoch."] # [doc = ""] # [doc = " Defaults to the current time. The caller may set this to the commit time if available."] pub modification_time : gix_date :: SecondsSinceUnixEpoch , }
};
}
