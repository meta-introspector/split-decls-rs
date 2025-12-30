// Generated macro for impl_118 (impl)
macro_rules! Depcrate_abi_exampleimpl_118 {
() => {
// Module: crate::abi_example
// Provides: {"impl_118"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl < T : AbiExample > AbiExample for boxcar :: Vec < T > { fn example () -> Self { println ! ("AbiExample for (boxcar::Vec): {}" , type_name ::< Self > ()) ; let vec = boxcar :: Vec :: new () ; vec . push (T :: example ()) ; vec } }
};
}
