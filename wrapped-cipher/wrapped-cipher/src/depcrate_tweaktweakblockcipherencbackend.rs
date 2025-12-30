// Generated macro for TweakBlockCipherEncBackend (trait)
macro_rules! Depcrate_tweakTweakBlockCipherEncBackend {
() => {
// Module: crate::tweak
// Provides: {"TweakBlockCipherEncBackend"}
// Dependencies: {}
# [doc = " Trait implemented by block cipher mode encryption backends."] pub trait TweakBlockCipherEncBackend : BlockSizeUser + TweakSizeUser { # [doc = " Encrypt single inout block."] fn encrypt_block_inout (& self , tweak : & Tweak < Self > , block : InOut < '_ , '_ , Block < Self > >) ; # [doc = " Encrypt single block in-place."] # [inline] fn encrypt_block (& self , tweak : & Tweak < Self > , block : & mut Block < Self >) { self . encrypt_block_inout (tweak , block . into ()) ; } # [doc = " Encrypt `in_block` and write result to `out_block`."] # [inline] fn encrypt_block_b2b (& self , tweak : & Tweak < Self > , in_block : & Block < Self > , out_block : & mut Block < Self > ,) { self . encrypt_block_inout (tweak , (in_block , out_block) . into ()) ; } }
};
}
