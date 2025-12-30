// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl < C > KeySizeUser for RetailMac < C > where C : BlockCipherEncrypt + BlockCipherDecrypt + Clone , < C as BlockSizeUser > :: BlockSize : Mul < U2 > , Prod < < C as BlockSizeUser > :: BlockSize , U2 > : ArraySize , { type KeySize = Prod < < C as BlockSizeUser > :: BlockSize , U2 > ; }
};
}
