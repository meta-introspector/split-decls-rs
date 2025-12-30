// Generated macro for CrateRunEnv (struct)
macro_rules! Depcrate_utilCrateRunEnv {
() => {
// Module: crate::util
// Provides: {"CrateRunEnv"}
// Dependencies: {}
# [doc = " The information to run a crate with the given environment."] # [derive (Clone , Serialize , Deserialize)] pub struct CrateRunEnv { # [doc = " The command-line arguments."] pub args : Vec < String > , # [doc = " The environment."] pub env : Vec < (OsString , OsString) > , # [doc = " The current working directory."] pub current_dir : OsString , # [doc = " The contents passed via standard input."] pub stdin : Vec < u8 > , }
};
}
