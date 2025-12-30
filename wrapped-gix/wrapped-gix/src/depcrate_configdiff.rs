// Generated macro for diff (module)
macro_rules! Depcrate_configdiff {
() => {
// Module: crate::config
// Provides: {"diff"}
// Dependencies: {}
# [doc = ""] pub mod diff { # [doc = ""] pub mod algorithm { use crate :: bstr :: BString ; # [doc = " The error produced when obtaining `diff.algorithm`."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Unknown diff algorithm named '{name}'")] Unknown { name : BString } , # [error ("The '{name}' algorithm is not yet implemented")] Unimplemented { name : BString } , } } # [doc = ""] pub mod pipeline_options { # [doc = " The error produced when obtaining options needed to fill in [gix_diff::blob::pipeline::Options]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] FilesystemCapabilities (# [from] crate :: config :: boolean :: Error) , # [error (transparent)] BigFileThreshold (# [from] crate :: config :: unsigned_integer :: Error) , } } # [doc = ""] pub mod drivers { use crate :: bstr :: BString ; # [doc = " The error produced when obtaining a list of [Drivers](gix_diff::blob::Driver)."] # [derive (Debug , thiserror :: Error)] # [error ("Failed to parse value of 'diff.{name}.{attribute}'")] pub struct Error { # [doc = " The name of the driver."] pub name : BString , # [doc = " The name of the attribute we tried to parse."] pub attribute : & 'static str , # [doc = " The actual error that occurred."] pub source : Box < dyn std :: error :: Error + Send + Sync + 'static > , } } }
};
}
