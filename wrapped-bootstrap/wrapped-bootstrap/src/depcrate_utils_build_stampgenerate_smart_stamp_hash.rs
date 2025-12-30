// Generated macro for generate_smart_stamp_hash (function)
macro_rules! Depcrate_utils_build_stampgenerate_smart_stamp_hash {
() => {
// Module: crate::utils::build_stamp
// Provides: {"generate_smart_stamp_hash"}
// Dependencies: {}
# [doc = " Computes a hash representing the state of a repository/submodule and additional input."] # [doc = ""] # [doc = " It uses `git diff` for the actual changes, and `git status` for including the untracked"] # [doc = " files in the specified directory. The additional input is also incorporated into the"] # [doc = " computation of the hash."] # [doc = ""] # [doc = " # Parameters"] # [doc = ""] # [doc = " - `dir`: A reference to the directory path of the target repository/submodule."] # [doc = " - `additional_input`: An additional input to be included in the hash."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " In case of errors during `git` command execution (e.g., in tarball sources), default values"] # [doc = " are used to prevent panics."] pub fn generate_smart_stamp_hash (builder : & Builder < '_ > , dir : & Path , additional_input : & str ,) -> String { let diff = helpers :: git (Some (dir)) . allow_failure () . arg ("diff") . arg (".") . run_capture_stdout (builder) . stdout_if_ok () . unwrap_or_default () ; let status = helpers :: git (Some (dir)) . allow_failure () . arg ("status") . arg (".") . arg ("--porcelain") . arg ("-z") . arg ("--untracked-files=normal") . run_capture_stdout (builder) . stdout_if_ok () . unwrap_or_default () ; let mut hasher = sha2 :: Sha256 :: new () ; hasher . update (diff) ; hasher . update (status) ; hasher . update (additional_input) ; hex_encode (hasher . finalize () . as_slice ()) }
};
}
