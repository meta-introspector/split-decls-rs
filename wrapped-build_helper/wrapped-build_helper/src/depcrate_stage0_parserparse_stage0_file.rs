// Generated macro for parse_stage0_file (function)
macro_rules! Depcrate_stage0_parserparse_stage0_file {
() => {
// Module: crate::stage0_parser
// Provides: {"parse_stage0_file"}
// Dependencies: {}
pub fn parse_stage0_file () -> Stage0 { let stage0_content = include_str ! ("../../stage0") ; let mut stage0 = Stage0 :: default () ; for line in stage0_content . lines () { let line = line . trim () ; if line . is_empty () { continue ; } if line . starts_with ('#') { continue ; } let (key , value) = line . split_once ('=') . unwrap () ; match key { "dist_server" => stage0 . config . dist_server = value . to_owned () , "artifacts_server" => stage0 . config . artifacts_server = value . to_owned () , "artifacts_with_llvm_assertions_server" => { stage0 . config . artifacts_with_llvm_assertions_server = value . to_owned () } "git_merge_commit_email" => stage0 . config . git_merge_commit_email = value . to_owned () , "nightly_branch" => stage0 . config . nightly_branch = value . to_owned () , "compiler_date" => stage0 . compiler . date = value . to_owned () , "compiler_version" => stage0 . compiler . version = value . to_owned () , "rustfmt_date" => { stage0 . rustfmt . get_or_insert (VersionMetadata :: default ()) . date = value . to_owned () ; } "rustfmt_version" => { stage0 . rustfmt . get_or_insert (VersionMetadata :: default ()) . version = value . to_owned () ; } dist if dist . starts_with ("dist") => { stage0 . checksums_sha256 . insert (key . to_owned () , value . to_owned ()) ; } unsupported => { println ! ("'{unsupported}' field is not supported.") ; } } } stage0 }
};
}
