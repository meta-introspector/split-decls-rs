// Generated macro for ForDeletionMode (enum)
macro_rules! Depcrate_walkForDeletionMode {
() => {
// Module: crate::walk
// Provides: {"ForDeletionMode"}
// Dependencies: {}
# [doc = " When the walk is for deletion, assure that we don't collapse directories that have precious files in"] # [doc = " them, and otherwise assure that no entries are observable that shouldn't be deleted."] # [derive (Default , Debug , Clone , Copy , PartialEq , Eq , Hash , Ord , PartialOrd)] pub enum ForDeletionMode { # [doc = " We will stop traversing into ignored directories which may save a lot of time, but also may include nested repositories"] # [doc = " which might end up being deleted."] # [default] IgnoredDirectoriesCanHideNestedRepositories , # [doc = " Instead of skipping over ignored directories entirely, we will dive in and find ignored non-bare repositories"] # [doc = " so these are emitted separately and prevent collapsing. These are assumed to be a directory with `.git` inside."] # [doc = " Only relevant when ignored entries are emitted."] FindNonBareRepositoriesInIgnoredDirectories , # [doc = " This is a more expensive form of the above variant as it finds all repositories, bare or non-bare."] FindRepositoriesInIgnoredDirectories , }
};
}
