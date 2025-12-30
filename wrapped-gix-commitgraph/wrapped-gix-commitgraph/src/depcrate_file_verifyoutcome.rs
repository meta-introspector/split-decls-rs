// Generated macro for Outcome (struct)
macro_rules! Depcrate_file_verifyOutcome {
() => {
// Module: crate::file::verify
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " The positive result of [`File::traverse()`] providing some statistical information."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize , serde :: Serialize))] pub struct Outcome { # [doc = " The largest encountered [`file::Commit`] generation number."] pub max_generation : u32 , # [doc = " The smallest encountered [`file::Commit`] generation number."] pub min_generation : u32 , # [doc = " The largest number of parents in a single [`file::Commit`]."] pub max_parents : u32 , # [doc = " The total number of [`commits`][file::Commit]s seen in the iteration."] pub num_commits : u32 , # [doc = " A mapping of `N -> number of commits with N parents`."] pub parent_counts : HashMap < u32 , u32 > , }
};
}
