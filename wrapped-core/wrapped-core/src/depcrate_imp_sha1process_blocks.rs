// Generated macro for process_blocks (function)
macro_rules! Depcrate_imp_sha1process_blocks {
() => {
// Module: crate::imp::sha1
// Provides: {"process_blocks"}
// Dependencies: {}
const fn process_blocks (mut blocks : Blocks , data : & ConstBuffer , mut len : u64 , mut state : [u32 ; 5] ,) -> (Blocks , u64 , [u32 ; 5]) { const fn as_block (input : & ConstBuffer , offset : usize) -> [u32 ; 16] { let mut result = [0u32 ; 16] ; let mut i = 0 ; while i != 16 { let off = offset + (i * 4) ; result [i] = (input . get (off + 3) as u32) | ((input . get (off + 2) as u32) << 8) | ((input . get (off + 1) as u32) << 16) | ((input . get (off) as u32) << 24) ; i += 1 ; } result } const fn clone_from_slice_64 (mut data : [u8 ; 64] , slice : & [u8] , offset : usize , num_elems : usize ,) -> [u8 ; 64] { let mut i = 0 ; while i < num_elems { data [i] = slice [offset + i] ; i += 1 ; } data } let mut i = 0 ; while i < data . len () { if data . len () - i >= 64 { let chunk_block = as_block (data , i) ; len += 64 ; state = process_state (state , chunk_block) ; i += 64 ; } else { let num_elems = data . len () - i ; blocks . data = clone_from_slice_64 (blocks . data , & data . data , i , num_elems) ; blocks . len = num_elems as u32 ; break ; } } (blocks , len , state) }
};
}
