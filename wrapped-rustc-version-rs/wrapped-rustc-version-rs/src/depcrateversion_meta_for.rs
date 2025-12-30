// Generated macro for version_meta_for (function)
macro_rules! Depcrateversion_meta_for {
() => {
// Module: crate
// Provides: {"version_meta_for"}
// Dependencies: {}
# [doc = " Parses a \"rustc -vV\" output string and returns"] # [doc = " the SemVer version and additional metadata"] # [doc = " like the git short hash and build date."] pub fn version_meta_for (verbose_version_string : & str) -> Result < VersionMeta > { let mut map = HashMap :: new () ; for (i , line) in verbose_version_string . lines () . enumerate () { if i == 0 { map . insert ("short" , line) ; continue ; } let mut parts = line . splitn (2 , ": ") ; let key = match parts . next () { Some (key) => key , None => continue , } ; if let Some (value) = parts . next () { map . insert (key , value) ; } } let short_version_string = expect_key ("short" , & map) ? ; let host = expect_key ("host" , & map) ? ; let release = expect_key ("release" , & map) ? ; let semver : Version = release . parse () ? ; let channel = match semver . pre . split ('.') . next () . unwrap () { "" => Channel :: Stable , "dev" => Channel :: Dev , "beta" => Channel :: Beta , "nightly" => Channel :: Nightly , x => return Err (Error :: UnknownPreReleaseTag (x . to_owned ())) , } ; let commit_hash = expect_key_or_unknown ("commit-hash" , & map) ? ; let commit_date = expect_key_or_unknown ("commit-date" , & map) ? ; let build_date = map . get ("build-date") . filter (| & v | * v != "unknown") . map (| & v | String :: from (v)) ; let llvm_version = match map . get ("LLVM version") { Some (& v) => Some (v . parse () ?) , None => None , } ; Ok (VersionMeta { semver , commit_hash , commit_date , build_date , channel , host , short_version_string , llvm_version , }) }
};
}
