// Generated macro for remote (module)
macro_rules! Depcrate_headremote {
() => {
// Module: crate::head
// Provides: {"remote"}
// Dependencies: {}
mod remote { use super :: Head ; use crate :: { remote , Remote } ; # [doc = " Remote"] impl < 'repo > Head < 'repo > { # [doc = " Return the remote with which the currently checked our reference can be handled as configured by `branch.<name>.remote|pushRemote`"] # [doc = " or fall back to the non-branch specific remote configuration. `None` is returned if the head is detached or unborn, so there is"] # [doc = " no branch specific remote."] # [doc = ""] # [doc = " This is equivalent to calling [`Reference::remote(…)`][crate::Reference::remote()] and"] # [doc = " [`Repository::remote_default_name()`][crate::Repository::remote_default_name()] in order."] # [doc = ""] # [doc = " Combine it with [`Repository::find_default_remote()`][crate::Repository::find_default_remote()] as fallback to"] # [doc = " handle detached heads, i.e. obtain a remote even in case of detached heads,"] # [doc = " or call [`Repository::find_fetch_remote(…)`](crate::Repository::find_fetch_remote()) for the highest-level way of finding"] # [doc = " the right remote, just like `git fetch` does."] pub fn into_remote (self , direction : remote :: Direction ,) -> Option < Result < Remote < 'repo > , remote :: find :: existing :: Error > > { let repo = self . repo ; self . try_into_referent () ? . remote (direction) . or_else (| | repo . find_default_remote (direction)) } } }
};
}
