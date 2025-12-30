// Generated macro for IgnoreMatchInner (enum)
macro_rules! Depcrate_dirIgnoreMatchInner {
() => {
// Module: crate::dir
// Provides: {"IgnoreMatchInner"}
// Dependencies: {}
# [doc = " IgnoreMatchInner describes precisely where the match information came from."] # [doc = " This is private to allow expansion to more matchers in the future."] # [derive (Clone , Debug)] # [allow (dead_code)] enum IgnoreMatchInner < 'a > { Override (overrides :: Glob < 'a >) , Gitignore (& 'a gitignore :: Glob) , Types (types :: Glob < 'a >) , Hidden , }
};
}
