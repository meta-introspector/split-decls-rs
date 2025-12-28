macro_rules! deps {
    () => {
        Kind!();
        Store!();
    };
}

macro_rules! store {
    () => {
        deps!();
        # [doc = ""] pub mod store { # [doc = ""] pub mod init { # [doc = " Options for use during [initialization](crate::file::Store::at)."] # [derive (Debug , Copy , Clone , Default)] pub struct Options { # [doc = " How to write the ref-log."] pub write_reflog : super :: WriteReflog , # [doc = " The kind of hash to expect in"] pub object_hash : gix_hash :: Kind , # [doc = " The equivalent of `core.precomposeUnicode`."] pub precompose_unicode : bool , # [doc = " If `true`, we will avoid reading from or writing to references that contains Windows device names"] # [doc = " to avoid side effects. This only needs to be `true` on Windows, but can be `true` on other platforms"] # [doc = " if they need to remain compatible with Windows."] pub prohibit_windows_device_names : bool , } } # [doc = " The way a file store handles the reflog"] # [derive (Default , Debug , PartialOrd , PartialEq , Ord , Eq , Hash , Clone , Copy)] pub enum WriteReflog { # [doc = " Always write the reflog for all references for ref edits, unconditionally."] Always , # [doc = " Write a ref log for ref edits according to the standard rules."] # [default] Normal , # [doc = " Never write a ref log."] Disable , } # [doc = " A thread-local handle for interacting with a [`Store`][crate::Store] to find and iterate references."] # [derive (Clone)] # [allow (dead_code)] pub (crate) struct Handle { # [doc = " A way to access shared state with the requirement that interior mutability doesn't leak or is incorporated into error types"] # [doc = " if it could. The latter can't happen if references to said internal aren't ever returned."] state : handle :: State , } # [allow (dead_code)] pub (crate) enum State { Loose { store : file :: Store } , } pub (crate) mod general ; # [doc = ""] # [path = "general/handle/mod.rs"] mod handle ; pub use handle :: find ; use crate :: file ; }
    };
}

store!();