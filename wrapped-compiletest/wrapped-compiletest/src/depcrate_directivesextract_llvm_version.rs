// Generated macro for extract_llvm_version (function)
macro_rules! Depcrate_directivesextract_llvm_version {
() => {
// Module: crate::directives
// Provides: {"extract_llvm_version"}
// Dependencies: {}
# [doc = " Given an llvm version string that looks like `1.2.3-rc1`, extract as semver. Note that this"] # [doc = " accepts more than just strict `semver` syntax (as in `major.minor.patch`); this permits omitting"] # [doc = " minor and patch version components so users can write e.g. `//@ min-llvm-version: 19` instead of"] # [doc = " having to write `//@ min-llvm-version: 19.0.0`."] # [doc = ""] # [doc = " Currently panics if the input string is malformed, though we really should not use panic as an"] # [doc = " error handling strategy."] # [doc = ""] # [doc = " FIXME(jieyouxu): improve error handling"] pub fn extract_llvm_version (version : & str) -> Version { let version = version . trim () ; let uninterested = | c : char | ! c . is_ascii_digit () && c != '.' ; let version_without_suffix = match version . split_once (uninterested) { Some ((prefix , _suffix)) => prefix , None => version , } ; let components : Vec < u64 > = version_without_suffix . split ('.') . map (| s | s . parse () . expect ("llvm version component should consist of only digits")) . collect () ; match & components [..] { [major] => Version :: new (* major , 0 , 0) , [major , minor] => Version :: new (* major , * minor , 0) , [major , minor , patch] => Version :: new (* major , * minor , * patch) , _ => panic ! ("malformed llvm version string, expected only 1-3 components: {version}") , } }
};
}
