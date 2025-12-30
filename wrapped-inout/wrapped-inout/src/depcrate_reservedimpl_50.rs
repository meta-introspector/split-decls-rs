// Generated macro for impl_50 (impl)
macro_rules! Depcrate_reservedimpl_50 {
() => {
// Module: crate::reserved
// Provides: {"impl_50"}
// Dependencies: {}
# [cfg (feature = "block-padding")] impl < 'inp , 'out > InOutBufReserved < 'inp , 'out , u8 > { # [doc = " Transform buffer into [`PaddedInOutBuf`] using padding algorithm `P`."] # [inline (always)] pub fn into_padded_blocks < P , BS > (self) -> Result < PaddedInOutBuf < 'inp , 'out , BS > , PadError > where P : Padding , BS : ArraySize , { let bs = BS :: USIZE ; let blocks_len = self . in_len / bs ; use block_padding :: PaddedData ; let (blocks , tail_block) = match P :: pad_detached (self . get_in ()) { PaddedData :: Pad { blocks , tail_block } => (blocks , Some (tail_block)) , PaddedData :: NoPad { blocks } => (blocks , None) , PaddedData :: Error => return Err (PadError) , } ; assert_eq ! (blocks . len () , blocks_len) ; let out_len = self . out_len ; let (in_ptr , out_ptr) = self . into_raw () ; let blocks = unsafe { InOutBuf :: from_raw (in_ptr . cast :: < Array < u8 , BS > > () , out_ptr . cast :: < Array < u8 , BS > > () , blocks_len ,) } ; let Some (tail_block) = tail_block else { let tail_inout = None ; return Ok (PaddedInOutBuf { blocks , tail_inout }) ; } ; let blocks_byte_len = blocks_len * bs ; let reserve_len = out_len - blocks_byte_len ; if reserve_len < tail_block . len () { return Err (PadError) ; } let tail_out : & mut Array < u8 , BS > = unsafe { let tail_out_ptr = out_ptr . add (blocks_byte_len) ; & mut * (tail_out_ptr . cast ()) } ; let tail_inout = Some ((tail_block , tail_out)) ; Ok (PaddedInOutBuf { blocks , tail_inout }) } }
};
}
