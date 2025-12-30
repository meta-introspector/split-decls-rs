// Generated macro for TweakBlockCipherDecrypt (trait)
macro_rules! Depcrate_tweakTweakBlockCipherDecrypt {
() => {
// Module: crate::tweak
// Provides: {"TweakBlockCipherDecrypt"}
// Dependencies: {}
# [doc = " Decrypt-only functionality for tweakable block ciphers."] pub trait TweakBlockCipherDecrypt : BlockSizeUser + TweakSizeUser + Sized { # [doc = " Decrypt data using backend provided to the rank-2 closure."] fn decrypt_with_backend (& self , f : impl TweakBlockCipherDecClosure < BlockSize = Self :: BlockSize , TweakSize = Self :: TweakSize > ,) ; # [doc = " Decrypt single `inout` block."] # [inline] fn decrypt_block_inout (& self , tweak : & Tweak < Self > , block : InOut < '_ , '_ , Block < Self > >) { self . decrypt_with_backend (ctx :: BlockCtx { tweak , block }) ; } # [doc = " Decrypt single block in-place."] # [inline] fn decrypt_block (& self , tweak : & Tweak < Self > , block : & mut Block < Self >) { self . decrypt_block_inout (tweak , block . into ()) ; } # [doc = " Decrypt `in_block` and write result to `out_block`."] # [inline] fn decrypt_block_b2b (& self , tweak : & Tweak < Self > , in_block : & Block < Self > , out_block : & mut Block < Self > ,) { self . decrypt_block_inout (tweak , (in_block , out_block) . into ()) ; } }
};
}
