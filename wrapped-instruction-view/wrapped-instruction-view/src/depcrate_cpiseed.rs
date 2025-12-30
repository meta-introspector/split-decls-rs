// Generated macro for Seed (struct)
macro_rules! Depcrate_cpiSeed {
() => {
// Module: crate::cpi
// Provides: {"Seed"}
// Dependencies: {}
# [doc = " Represents a signer seed."] # [doc = ""] # [doc = " This struct contains the same information as a `[u8]`, but"] # [doc = " has the memory layout as expected by `sol_invoke_signed_c`"] # [doc = " syscall."] # [repr (C)] # [derive (Debug , Clone)] pub struct Seed < 'a > { # [doc = " Seed bytes."] pub (crate) seed : * const u8 , # [doc = " Length of the seed bytes."] pub (crate) len : u64 , # [doc = " The pointer to the seed bytes is only valid while the `&'a [u8]` lives. Instead"] # [doc = " of holding a reference to the actual `[u8]`, which would increase the size of the"] # [doc = " type, we claim to hold a reference without actually holding one using a"] # [doc = " `PhantomData<&'a [u8]>`."] _bytes : PhantomData < & 'a [u8] > , }
};
}
