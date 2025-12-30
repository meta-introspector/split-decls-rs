// Generated macro for SigSet (struct)
macro_rules! Depcrate_machinst_abiSigSet {
() => {
// Module: crate::machinst::abi
// Provides: {"SigSet"}
// Dependencies: {}
# [doc = " A (mostly) deduplicated set of ABI signatures."] # [doc = ""] # [doc = " We say \"mostly\" because we do not dedupe between signatures interned via"] # [doc = " `ir::SigRef` (direct and indirect calls; the vast majority of signatures in"] # [doc = " this set) vs via `ir::Signature` (the callee itself and libcalls). Doing"] # [doc = " this final bit of deduplication would require filling out the"] # [doc = " `ir_signature_to_abi_sig`, which is a bunch of allocations (not just the"] # [doc = " hash map itself but params and returns vecs in each signature) that we want"] # [doc = " to avoid."] # [doc = ""] # [doc = " In general, prefer using the `ir::SigRef`-taking methods to the"] # [doc = " `ir::Signature`-taking methods when you can get away with it, as they don't"] # [doc = " require cloning non-copy types that will trigger heap allocations."] # [doc = ""] # [doc = " This type can be indexed by `Sig` to access its associated `SigData`."] pub struct SigSet { # [doc = " Interned `ir::Signature`s that we already have an ABI signature for."] ir_signature_to_abi_sig : FxHashMap < ir :: Signature , Sig > , # [doc = " Interned `ir::SigRef`s that we already have an ABI signature for."] ir_sig_ref_to_abi_sig : SecondaryMap < ir :: SigRef , Option < Sig > > , # [doc = " A single, shared allocation for all `ABIArg`s used by all"] # [doc = " `SigData`s. Each `SigData` references its args/rets via indices into"] # [doc = " this allocation."] abi_args : Vec < ABIArg > , # [doc = " The actual ABI signatures, keyed by `Sig`."] sigs : PrimaryMap < Sig , SigData > , }
};
}
