// Generated macro for TweakBlockCipherEncrypt (trait)
macro_rules! Depcrate_tweakTweakBlockCipherEncrypt {
() => {
// Module: crate::tweak
// Provides: {"TweakBlockCipherEncrypt"}
// Dependencies: {}
# [doc = " Encrypt-only functionality for tweakable block ciphers."] pub trait TweakBlockCipherEncrypt : BlockSizeUser + TweakSizeUser + Sized { # [doc = " Encrypt data using backend provided to the rank-2 closure."] fn encrypt_with_backend (& self , f : impl TweakBlockCipherEncClosure < BlockSize = Self :: BlockSize , TweakSize = Self :: TweakSize > ,) ; # [doc = " Encrypt single `inout` block."] # [inline] fn encrypt_block_inout (& self , tweak : & Tweak < Self > , block : InOut < '_ , '_ , Block < Self > >) { self . encrypt_with_backend (ctx :: BlockCtx { tweak , block }) ; } # [doc = " Encrypt single block in-place."] # [inline] fn encrypt_block (& self , tweak : & Tweak < Self > , block : & mut Block < Self >) { self . encrypt_block_inout (tweak , block . into ()) ; } # [doc = " Encrypt `in_block` and write result to `out_block`."] # [inline] fn encrypt_block_b2b (& self , tweak : & Tweak < Self > , in_block : & Block < Self > , out_block : & mut Block < Self > ,) { self . encrypt_block_inout (tweak , (in_block , out_block) . into ()) ; } }
};
}
