// Generated macro for VersionMeta (struct)
macro_rules! DepcrateVersionMeta {
() => {
// Module: crate
// Provides: {"VersionMeta"}
// Dependencies: {}
# [doc = " Rustc version plus metadata like git short hash and build date."] # [derive (Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct VersionMeta { # [doc = " Version of the compiler"] pub semver : Version , # [doc = " Git short hash of the build of the compiler"] pub commit_hash : Option < String > , # [doc = " Commit date of the compiler"] pub commit_date : Option < String > , # [doc = " Build date of the compiler; this was removed between Rust 1.0.0 and 1.1.0."] pub build_date : Option < String > , # [doc = " Release channel of the compiler"] pub channel : Channel , # [doc = " Host target triple of the compiler"] pub host : String , # [doc = " Short version string of the compiler"] pub short_version_string : String , # [doc = " Version of LLVM used by the compiler"] pub llvm_version : Option < LlvmVersion > , }
};
}
