use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An enumeration of the possible priority levels of a config file.
///
/// The levels corresponding to the escalation logic (higher to lower) when
/// searching for config entries.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum ConfigLevel {
    /// System-wide on Windows, for compatibility with portable git
    ProgramData = 1,
    /// System-wide configuration file, e.g. /etc/gitconfig
    System,
    /// XDG-compatible configuration file, e.g. ~/.config/git/config
    XDG,
    /// User-specific configuration, e.g. ~/.gitconfig
    Global,
    /// Repository specific config, e.g. $PWD/.git/config
    Local,
    ///  Worktree specific configuration file, e.g. $GIT_DIR/config.worktree
    Worktree,
    /// Application specific configuration file
    App,
    /// Highest level available
    Highest = -1,
}
