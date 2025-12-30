// Generated macro for CfgState (enum)
macro_rules! Depcrate_cfgsCfgState {
() => {
// Module: crate::cfgs
// Provides: {"CfgState"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , Default , PartialEq)] enum CfgState { ShouldGate , # [doc = " Whether we emit a `cfg` or not is irrelevant, because it is already"] # [doc = " gated at a higher level."] # [default] AlreadyGated , Omit , }
};
}
