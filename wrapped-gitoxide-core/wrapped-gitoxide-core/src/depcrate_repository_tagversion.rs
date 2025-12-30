// Generated macro for Version (struct)
macro_rules! Depcrate_repository_tagVersion {
() => {
// Module: crate::repository::tag
// Provides: {"Version"}
// Dependencies: {}
# [doc = " `Version` is used to store multi-part version numbers. It does so in a rather naive way,"] # [doc = " only distinguishing between parts that can be parsed as an integer and those that cannot."] # [doc = ""] # [doc = " `Version` does not parse version numbers in any structure-aware way, so `v0.a` is parsed into"] # [doc = " `v`, `0`, `.a`."] # [doc = ""] # [doc = " Comparing two `Version`s comes down to comparing their `parts`. `parts` are either compared"] # [doc = " numerically or lexicographically, depending on whether they are an integer or not. That way,"] # [doc = " `v0.9` sorts before `v0.10` as one would expect from a version number."] # [doc = ""] # [doc = " When comparing versions of different lengths, shorter versions sort before longer ones (e.g.,"] # [doc = " `v1.0` < `v1.0.1`). String parts always sort before numeric parts when compared directly."] # [derive (Eq , PartialEq , Ord , PartialOrd)] struct Version { parts : Vec < VersionPart > , }
};
}
