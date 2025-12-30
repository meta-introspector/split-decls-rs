// Generated macro for impl_52 (impl)
macro_rules! Depcrate_reservedimpl_52 {
() => {
// Module: crate::reserved
// Provides: {"impl_52"}
// Dependencies: {}
# [cfg (feature = "block-padding")] impl < 'out , BS : ArraySize > PaddedInOutBuf < '_ , 'out , BS > { # [doc = " Get full blocks."] # [inline (always)] pub fn get_blocks (& mut self) -> InOutBuf < '_ , '_ , Array < u8 , BS > > { self . blocks . reborrow () } # [doc = " Get padded tail block."] # [doc = ""] # [doc = " Most padding implementations always return `Some`."] # [inline (always)] pub fn get_tail_block (& mut self) -> Option < InOut < '_ , '_ , Array < u8 , BS > > > { self . tail_inout . as_mut () . map (| (in_block , out_block) | { let in_block = & * in_block ; let out_block = & mut * * out_block ; InOut :: from ((in_block , out_block)) }) } # [doc = " Convert buffer into output slice."] # [inline (always)] pub fn into_out (self) -> & 'out [u8] { let total_blocks = if self . tail_inout . is_some () { self . blocks . len () + 1 } else { self . blocks . len () } ; let res_len = BS :: USIZE * total_blocks ; let (_ , out_ptr) = self . blocks . into_raw () ; unsafe { slice :: from_raw_parts (out_ptr as * const u8 , res_len) } } }
};
}
