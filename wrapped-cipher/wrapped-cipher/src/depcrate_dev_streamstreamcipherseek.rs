// Generated macro for StreamCipherSeek (trait)
macro_rules! Depcrate_dev_streamStreamCipherSeek {
() => {
// Module: crate::dev::stream
// Provides: {"StreamCipherSeek"}
// Dependencies: {}
# [doc = " Trait for seekable stream ciphers."] # [doc = ""] # [doc = " Methods of this trait are generic over the [`SeekNum`] trait,"] # [doc = " i.e. they can be used with `i32`, `u32`, `u64`, `u128`, and `usize`."] pub trait StreamCipherSeek { # [doc = " Try to get current keystream position in bytes."] # [doc = ""] # [doc = " Returns [`OverflowError`] if the position value can not be represented by type `T`."] fn try_current_pos < T : SeekNum > (& self) -> Result < T , OverflowError > ; # [doc = " Try to seek to the provided position in bytes."] # [doc = ""] # [doc = " Returns [`StreamCipherError`] if the position value is bigger than keystream length."] fn try_seek < T : SeekNum > (& mut self , pos : T) -> Result < () , StreamCipherError > ; # [doc = " Get current keystream position in bytes."] # [doc = ""] # [doc = " # Panics"] # [doc = " If the position value can not be represented by type `T`."] fn current_pos < T : SeekNum > (& self) -> T { self . try_current_pos () . unwrap () } # [doc = " Seek to the provided keystream position in bytes."] # [doc = ""] # [doc = " # Panics"] # [doc = " If the position value is bigger than keystream length."] fn seek < T : SeekNum > (& mut self , pos : T) { self . try_seek (pos) . unwrap () } }
};
}
