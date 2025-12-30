// Generated macro for ConfigLevel (enum)
macro_rules! DepcrateConfigLevel {
() => {
// Module: crate
// Provides: {"ConfigLevel"}
// Dependencies: {}
# [doc = " An enumeration of the possible priority levels of a config file."] # [doc = ""] # [doc = " The levels corresponding to the escalation logic (higher to lower) when"] # [doc = " searching for config entries."] # [derive (PartialEq , Eq , Debug , Copy , Clone)] pub enum ConfigLevel { # [doc = " System-wide on Windows, for compatibility with portable git"] ProgramData = 1 , # [doc = " System-wide configuration file, e.g. /etc/gitconfig"] System , # [doc = " XDG-compatible configuration file, e.g. ~/.config/git/config"] XDG , # [doc = " User-specific configuration, e.g. ~/.gitconfig"] Global , # [doc = " Repository specific config, e.g. $PWD/.git/config"] Local , # [doc = "  Worktree specific configuration file, e.g. $GIT_DIR/config.worktree"] Worktree , # [doc = " Application specific configuration file"] App , # [doc = " Highest level available"] Highest = - 1 , }
};
}
