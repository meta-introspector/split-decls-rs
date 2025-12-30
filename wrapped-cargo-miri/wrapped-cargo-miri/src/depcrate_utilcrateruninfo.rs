// Generated macro for CrateRunInfo (enum)
macro_rules! Depcrate_utilCrateRunInfo {
() => {
// Module: crate::util
// Provides: {"CrateRunInfo"}
// Dependencies: {}
# [doc = " The information Miri needs to run a crate. Stored as JSON when the crate is \"compiled\"."] # [derive (Serialize , Deserialize)] pub enum CrateRunInfo { # [doc = " Run it with the given environment."] RunWith (CrateRunEnv) , # [doc = " Skip it as Miri does not support interpreting such kind of crates."] SkipProcMacroTest , }
};
}
