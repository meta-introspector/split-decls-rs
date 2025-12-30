// Generated macro for TweakBlockCipherEncClosure (trait)
macro_rules! Depcrate_tweakTweakBlockCipherEncClosure {
() => {
// Module: crate::tweak
// Provides: {"TweakBlockCipherEncClosure"}
// Dependencies: {}
# [doc = " Trait for [`TweakBlockCipherEncBackend`] users."] # [doc = ""] # [doc = " This trait is used to define rank-2 closures."] pub trait TweakBlockCipherEncClosure : BlockSizeUser + TweakSizeUser { # [doc = " Execute closure with the provided block cipher backend."] fn call < B > (self , backend : & B) where B : TweakBlockCipherEncBackend < BlockSize = Self :: BlockSize , TweakSize = Self :: TweakSize > ; }
};
}
