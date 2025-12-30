// Generated macro for FoldHasher (struct)
macro_rules! Depcrate_fastFoldHasher {
() => {
// Module: crate::fast
// Provides: {"FoldHasher"}
// Dependencies: {}
# [doc = " A [`Hasher`] instance implementing foldhash, optimized for speed."] # [doc = ""] # [doc = " While you can create one directly with [`FoldHasher::with_seed`], you"] # [doc = " most likely want to use [`RandomState`], [`SeedableRandomState`] or"] # [doc = " [`FixedState`] to create [`FoldHasher`]s."] # [derive (Clone)] pub struct FoldHasher < 'a > { accumulator : u64 , sponge : u128 , sponge_len : u8 , seeds : & 'a [u64 ; 6] , }
};
}
