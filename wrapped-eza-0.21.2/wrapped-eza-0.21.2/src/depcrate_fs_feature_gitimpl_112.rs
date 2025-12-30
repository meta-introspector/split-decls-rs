// Generated macro for impl_112 (impl)
macro_rules! Depcrate_fs_feature_gitimpl_112 {
() => {
// Module: crate::fs::feature::git
// Provides: {"impl_112"}
// Dependencies: {}
impl GitContents { # [doc = " Assumes that the repository hasn’t been queried, and extracts it"] # [doc = " (consuming the value) if it has. This is needed because the entire"] # [doc = " enum variant gets replaced when a repo is queried (see above)."] fn inner_repo (self) -> git2 :: Repository { if let Self :: Before { repo } = self { repo } else { unreachable ! ("Tried to extract a non-Repository") } } }
};
}
