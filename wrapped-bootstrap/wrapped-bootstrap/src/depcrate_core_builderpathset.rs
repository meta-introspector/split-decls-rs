// Generated macro for PathSet (enum)
macro_rules! Depcrate_core_builderPathSet {
() => {
// Module: crate::core::builder
// Provides: {"PathSet"}
// Dependencies: {}
# [doc = " Collection of paths used to match a task rule."] # [derive (Debug , Clone , PartialOrd , Ord , PartialEq , Eq)] pub enum PathSet { # [doc = " A collection of individual paths or aliases."] # [doc = ""] # [doc = " These are generally matched as a path suffix. For example, a"] # [doc = " command-line value of `std` will match if `library/std` is in the"] # [doc = " set."] # [doc = ""] # [doc = " NOTE: the paths within a set should always be aliases of one another."] # [doc = " For example, `src/librustdoc` and `src/tools/rustdoc` should be in the same set,"] # [doc = " but `library/core` and `library/std` generally should not, unless there's no way (for that Step)"] # [doc = " to build them separately."] Set (BTreeSet < TaskPath >) , # [doc = " A \"suite\" of paths."] # [doc = ""] # [doc = " These can match as a path suffix (like `Set`), or as a prefix. For"] # [doc = " example, a command-line value of `tests/ui/abi/variadic-ffi.rs`"] # [doc = " will match `tests/ui`. A command-line value of `ui` would also"] # [doc = " match `tests/ui`."] Suite (TaskPath) , }
};
}
