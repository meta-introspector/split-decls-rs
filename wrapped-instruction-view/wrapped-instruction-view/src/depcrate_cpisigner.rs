// Generated macro for Signer (struct)
macro_rules! Depcrate_cpiSigner {
() => {
// Module: crate::cpi
// Provides: {"Signer"}
// Dependencies: {}
# [doc = " Represents a [program derived address][pda] (PDA) signer controlled by the"] # [doc = " calling program."] # [doc = ""] # [doc = " [pda]: https://solana.com/docs/core/cpi#program-derived-addresses"] # [repr (C)] # [derive (Debug , Clone)] pub struct Signer < 'a , 'b > { # [doc = " Signer seeds."] pub (crate) seeds : * const Seed < 'a > , # [doc = " Number of seeds."] pub (crate) len : u64 , # [doc = " The pointer to the seeds is only valid while the `&'b [Seed<'a>]` lives. Instead"] # [doc = " of holding a reference to the actual `[Seed<'a>]`, which would increase the size"] # [doc = " of the type, we claim to hold a reference without actually holding one using a"] # [doc = " `PhantomData<&'b [Seed<'a>]>`."] _seeds : PhantomData < & 'b [Seed < 'a >] > , }
};
}
