// Generated macro for impl_9 (impl)
macro_rules! Depcrate_decoderimpl_9 {
() => {
// Module: crate::decoder
// Provides: {"impl_9"}
// Dependencies: {}
impl LiteralDecoder { fn new (lc : u32 , lp : u32) -> Self { let coder = LiteralCoder :: new (lc , lp) ; let sub_decoders = vec ! [LiteralSubDecoder :: new () ; (1 << (lc + lp)) as _] ; Self { coder , sub_decoders , } } fn reset (& mut self) { for ele in self . sub_decoders . iter_mut () { ele . coder . reset () } } fn decode < R : RangeReader > (& mut self , coder : & mut LzmaCoder , lz : & mut LzDecoder , rc : & mut RangeDecoder < R > ,) -> crate :: Result < () > { let i = self . coder . get_sub_coder_index (lz . get_byte (0) as _ , lz . get_pos () as _) ; let d = & mut self . sub_decoders [i as usize] ; d . decode (coder , lz , rc) } }
};
}
