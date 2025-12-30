// Generated macro for Commits (type)
macro_rules! Depcrate_shallowCommits {
() => {
// Module: crate::shallow
// Provides: {"Commits"}
// Dependencies: {}
# [doc = " A lazily loaded and auto-updated list of commits which are at the shallow boundary (behind which there are no commits available),"] # [doc = " sorted to allow bisecting."] pub type Commits = gix_fs :: SharedFileSnapshot < Vec < gix_hash :: ObjectId > > ;
};
}
