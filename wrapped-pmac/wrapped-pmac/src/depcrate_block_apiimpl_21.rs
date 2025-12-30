// Generated macro for impl_21 (impl)
macro_rules! Depcrate_block_apiimpl_21 {
() => {
// Module: crate::block_api
// Provides: {"impl_21"}
// Dependencies: {}
impl < C : PmacCipher , const LC_SIZE : usize > UpdateCore for PmacCore < C , LC_SIZE > { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { struct Closure < 'a , C : PmacCipher , const LC_SIZE : usize > { state : & 'a mut PmacState < C , LC_SIZE > , blocks : & 'a [Block < C >] , } impl < C : PmacCipher , const LC_SIZE : usize > BlockSizeUser for Closure < '_ , C , LC_SIZE > { type BlockSize = C :: BlockSize ; } impl < C : PmacCipher , const LC_SIZE : usize > BlockCipherEncClosure for Closure < '_ , C , LC_SIZE > { # [inline (always)] fn call < B : BlockCipherEncBackend < BlockSize = Self :: BlockSize > > (self , backend : & B) { let Self { mut blocks , state } = self ; if B :: ParBlocksSize :: USIZE > 1 { let mut iter = blocks . chunks_exact (B :: ParBlocksSize :: USIZE) ; for chunk in & mut iter { let mut tmp = ParBlocks :: < B > :: try_from (chunk) . expect ("size mismatch") ; for block in tmp . iter_mut () { xor (block , state . next_offset ()) ; } backend . encrypt_par_blocks ((& mut tmp) . into ()) ; for t in tmp . iter () { xor (& mut state . tag , t) ; } } blocks = iter . remainder () ; } for block in blocks { let mut block = block . clone () ; xor (& mut block , state . next_offset ()) ; backend . encrypt_block ((& mut block) . into ()) ; xor (& mut state . tag , & block) ; } } } let Self { cipher , state } = self ; cipher . encrypt_with_backend (Closure { blocks , state }) } }
};
}
