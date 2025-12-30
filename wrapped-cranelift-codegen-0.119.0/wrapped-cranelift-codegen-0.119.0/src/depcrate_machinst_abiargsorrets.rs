// Generated macro for ArgsOrRets (enum)
macro_rules! Depcrate_machinst_abiArgsOrRets {
() => {
// Module: crate::machinst::abi
// Provides: {"ArgsOrRets"}
// Dependencies: {}
# [doc = " Are we computing information about arguments or return values? Much of the"] # [doc = " handling is factored out into common routines; this enum allows us to"] # [doc = " distinguish which case we're handling."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum ArgsOrRets { # [doc = " Arguments."] Args , # [doc = " Return values."] Rets , }
};
}
