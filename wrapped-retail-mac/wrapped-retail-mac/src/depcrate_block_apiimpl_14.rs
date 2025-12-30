// Generated macro for impl_14 (impl)
macro_rules! Depcrate_block_apiimpl_14 {
() => {
// Module: crate::block_api
// Provides: {"impl_14"}
// Dependencies: {}
impl < C > UpdateCore for RetailMacCore < C > where C : BlockCipherEncrypt + BlockCipherDecrypt + Clone , { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { struct Closure < 'a , N : BlockSizes > { state : & 'a mut Block < Self > , blocks : & 'a [Block < Self >] , } impl < N : BlockSizes > BlockSizeUser for Closure < '_ , N > { type BlockSize = N ; } impl < N : BlockSizes > BlockCipherEncClosure for Closure < '_ , N > { # [inline (always)] fn call < B : BlockCipherEncBackend < BlockSize = Self :: BlockSize > > (self , backend : & B) { for block in self . blocks { xor (self . state , block) ; backend . encrypt_block ((self . state) . into ()) ; } } } let Self { cipher , state , .. } = self ; cipher . encrypt_with_backend (Closure { state , blocks }) } }
};
}
