// Generated macro for TweakBlockCipherDecBackend (trait)
macro_rules! Depcrate_tweakTweakBlockCipherDecBackend {
() => {
// Module: crate::tweak
// Provides: {"TweakBlockCipherDecBackend"}
// Dependencies: {}
# [doc = " Trait implemented by block cipher mode decryption backends."] pub trait TweakBlockCipherDecBackend : BlockSizeUser + TweakSizeUser { # [doc = " Decrypt single inout block."] fn decrypt_block_inout (& self , tweak : & Tweak < Self > , block : InOut < '_ , '_ , Block < Self > >) ; # [doc = " Decrypt single block in-place."] # [inline] fn decrypt_block (& self , tweak : & Tweak < Self > , block : & mut Block < Self >) { self . decrypt_block_inout (tweak , block . into ()) ; } # [doc = " Decrypt `in_block` and write result to `out_block`."] # [inline] fn decrypt_block_b2b (& self , tweak : & Tweak < Self > , in_block : & Block < Self > , out_block : & mut Block < Self > ,) { self . decrypt_block_inout (tweak , (in_block , out_block) . into ()) ; } }
};
}
