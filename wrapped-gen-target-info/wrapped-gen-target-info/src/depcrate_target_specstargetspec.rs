// Generated macro for TargetSpec (struct)
macro_rules! Depcrate_target_specsTargetSpec {
() => {
// Module: crate::target_specs
// Provides: {"TargetSpec"}
// Dependencies: {}
# [derive (Debug , Deserialize)] # [serde (rename_all (deserialize = "kebab-case"))] pub struct TargetSpec { pub arch : String , pub llvm_target : String , # [doc = " link env to remove, mostly for apple"] pub link_env_remove : Option < Vec < String > > , # [doc = " link env to set, mostly for apple, e.g. `ZERO_AR_DATE=1`"] pub link_env : Option < Vec < String > > , pub os : Option < String > , # [doc = " `apple`, `pc`"] pub vendor : Option < String > , pub env : Option < String > , pub abi : Option < String > , pub pre_link_args : Option < PreLinkArgs > , }
};
}
