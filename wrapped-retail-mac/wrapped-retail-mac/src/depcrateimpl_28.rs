// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl < C > KeyInit for RetailMac < C > where C : BlockCipherEncrypt + BlockCipherDecrypt + Clone + KeyInit , < C as BlockSizeUser > :: BlockSize : Mul < U2 > , Prod < < C as BlockSizeUser > :: BlockSize , U2 > : ArraySize , { # [inline (always)] fn new (key : & Key < Self >) -> Self { Self { core : KeyInit :: new (key) , buffer : Default :: default () , } } # [inline (always)] fn new_from_slice (key : & [u8]) -> Result < Self , InvalidLength > { KeyInit :: new_from_slice (key) . map (| core | Self { core , buffer : Default :: default () , }) } }
};
}
