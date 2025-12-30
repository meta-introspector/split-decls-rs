// Generated macro for impl_36 (impl)
macro_rules! Depcrate_lz_hc4impl_36 {
() => {
// Module: crate::lz::hc4
// Provides: {"impl_36"}
// Dependencies: {}
impl Hc4 { pub (crate) fn get_mem_usage (dict_size : u32) -> u32 { Hash234 :: get_mem_usage (dict_size) + dict_size / (1024 / 4) + 10 } pub (crate) fn new (dict_size : u32 , nice_len : u32 , depth_limit : i32) -> Self { let chain = vec ! [0 ; dict_size as usize + 1] ; Self { hash : Hash234 :: new (dict_size) , chain , depth_limit : if depth_limit > 0 { depth_limit } else { 4 + nice_len as i32 / 4 } , cyclic_size : dict_size as i32 + 1 , cyclic_pos : - 1 , lz_pos : dict_size as i32 + 1 , } } fn move_pos (& mut self , encoder : & mut LzEncoderData) -> i32 { let avail = encoder . move_pos (4 , 4) ; if avail != 0 { self . lz_pos += 1 ; if self . lz_pos == 0x7FFFFFFF { let norm_offset = 0x7FFFFFFF - self . cyclic_size ; self . hash . normalize (norm_offset) ; LzEncoder :: normalize (& mut self . chain , norm_offset) ; self . lz_pos = self . lz_pos . wrapping_sub (norm_offset) ; } self . cyclic_pos += 1 ; if self . cyclic_pos == self . cyclic_size { self . cyclic_pos = 0 ; } } avail } }
};
}
