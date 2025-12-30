// Generated macro for seeds (macro)
macro_rules! Depcrate_cpiseeds {
() => {
// Module: crate::cpi
// Provides: {"seeds"}
// Dependencies: {}
# [doc = " Convenience macro for constructing a `[Seed; N]` array from a list of seeds"] # [doc = " to create a [`Signer`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Creating seeds array and signer for a PDA with a single seed and bump value:"] # [doc = " ```"] # [doc = " use solana_address::Address;"] # [doc = " use solana_instruction_view::{cpi::Signer, seeds};"] # [doc = ""] # [doc = " let pda_bump = 0xffu8;"] # [doc = " let pda_ref = &[pda_bump];"] # [doc = " let example_key = Address::default();"] # [doc = " let seeds = seeds!(b\"seed\", example_key.as_ref(), pda_ref);"] # [doc = " let signer = Signer::from(&seeds);"] # [doc = " ```"] # [macro_export] macro_rules ! seeds { ($ ($ seed : expr) ,*) => { [$ ($ crate :: cpi :: Seed :: from ($ seed) ,) *] } ; }
};
}
