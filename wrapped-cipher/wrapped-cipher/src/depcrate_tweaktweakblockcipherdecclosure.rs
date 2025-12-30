// Generated macro for TweakBlockCipherDecClosure (trait)
macro_rules! Depcrate_tweakTweakBlockCipherDecClosure {
() => {
// Module: crate::tweak
// Provides: {"TweakBlockCipherDecClosure"}
// Dependencies: {}
# [doc = " Trait for [`TweakBlockCipherDecBackend`] users."] # [doc = ""] # [doc = " This trait is used to define rank-2 closures."] pub trait TweakBlockCipherDecClosure : BlockSizeUser + TweakSizeUser { # [doc = " Execute closure with the provided block cipher backend."] fn call < B > (self , backend : & B) where B : TweakBlockCipherDecBackend < BlockSize = Self :: BlockSize , TweakSize = Self :: TweakSize > ; }
};
}
