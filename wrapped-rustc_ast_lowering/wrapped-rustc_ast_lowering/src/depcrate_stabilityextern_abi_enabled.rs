// Generated macro for extern_abi_enabled (function)
macro_rules! Depcrate_stabilityextern_abi_enabled {
() => {
// Module: crate::stability
// Provides: {"extern_abi_enabled"}
// Dependencies: {}
pub (crate) fn extern_abi_enabled (features : & rustc_feature :: Features , span : Span , abi : ExternAbi ,) -> Result < () , UnstableAbi > { extern_abi_stability (abi) . or_else (| unstable @ UnstableAbi { feature , .. } | { if features . enabled (feature) || span . allows_unstable (feature) { Ok (()) } else { Err (unstable) } }) }
};
}
