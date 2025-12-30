// Generated macro for CloneLocal (enum)
macro_rules! Depcrate_buildCloneLocal {
() => {
// Module: crate::build
// Provides: {"CloneLocal"}
// Dependencies: {}
# [doc = " Options that can be passed to `RepoBuilder::clone_local`."] # [derive (Clone , Copy)] pub enum CloneLocal { # [doc = " Auto-detect (default)"] # [doc = ""] # [doc = " Here libgit2 will bypass the git-aware transport for local paths, but"] # [doc = " use a normal fetch for `file://` URLs."] Auto = raw :: GIT_CLONE_LOCAL_AUTO as isize , # [doc = " Bypass the git-aware transport even for `file://` URLs."] Local = raw :: GIT_CLONE_LOCAL as isize , # [doc = " Never bypass the git-aware transport"] None = raw :: GIT_CLONE_NO_LOCAL as isize , # [doc = " Bypass the git-aware transport, but don't try to use hardlinks."] NoLinks = raw :: GIT_CLONE_LOCAL_NO_LINKS as isize , # [doc (hidden)] __Nonexhaustive = 0xff , }
};
}
