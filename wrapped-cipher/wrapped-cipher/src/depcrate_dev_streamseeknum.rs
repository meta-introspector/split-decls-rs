// Generated macro for SeekNum (trait)
macro_rules! Depcrate_dev_streamSeekNum {
() => {
// Module: crate::dev::stream
// Provides: {"SeekNum"}
// Dependencies: {}
# [doc = " Trait implemented for numeric types which can be used with the"] # [doc = " [`StreamCipherSeek`] trait."] # [doc = ""] # [doc = " This trait is implemented for `i32`, `u32`, `u64`, `u128`, and `usize`."] # [doc = " It is not intended to be implemented in third-party crates."] pub trait SeekNum : Sized { # [doc = " Try to get position for block number `block`, byte position inside"] # [doc = " block `byte`, and block size `bs`."] fn from_block_byte < T : StreamCipherCounter > (block : T , byte : u8 , bs : u8 ,) -> Result < Self , OverflowError > ; # [doc = " Try to get block number and bytes position for given block size `bs`."] fn into_block_byte < T : StreamCipherCounter > (self , bs : u8) -> Result < (T , u8) , OverflowError > ; }
};
}
