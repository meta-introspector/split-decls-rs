// Generated macro for should_fix_bins_and_dylibs (function)
macro_rules! Depcrate_core_downloadshould_fix_bins_and_dylibs {
() => {
// Module: crate::core::download
// Provides: {"should_fix_bins_and_dylibs"}
// Dependencies: {}
fn should_fix_bins_and_dylibs (patch_binaries_for_nix : Option < bool > , exec_ctx : & ExecutionContext ,) -> bool { let val = * SHOULD_FIX_BINS_AND_DYLIBS . get_or_init (| | { let uname = command ("uname") . allow_failure () . arg ("-s") . run_capture_stdout (exec_ctx) ; if uname . is_failure () { return false ; } let output = uname . stdout () ; if ! output . starts_with ("Linux") { return false ; } if let Some (explicit_value) = patch_binaries_for_nix { return explicit_value ; } let is_nixos = match File :: open ("/etc/os-release") { Err (e) if e . kind () == ErrorKind :: NotFound => false , Err (e) => panic ! ("failed to access /etc/os-release: {e}") , Ok (os_release) => BufReader :: new (os_release) . lines () . any (| l | { let l = l . expect ("reading /etc/os-release") ; matches ! (l . trim () , "ID=nixos" | "ID='nixos'" | "ID=\"nixos\"") }) , } ; if ! is_nixos { let in_nix_shell = env :: var ("IN_NIX_SHELL") ; if let Ok (in_nix_shell) = in_nix_shell { eprintln ! ("The IN_NIX_SHELL environment variable is `{in_nix_shell}`; \
                     you may need to set `patch-binaries-for-nix=true` in bootstrap.toml") ; } } is_nixos }) ; if val { eprintln ! ("INFO: You seem to be using Nix.") ; } val }
};
}
