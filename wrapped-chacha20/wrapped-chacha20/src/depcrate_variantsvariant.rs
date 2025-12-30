// Generated macro for Variant (trait)
macro_rules! Depcrate_variantsVariant {
() => {
// Module: crate::variants
// Provides: {"Variant"}
// Dependencies: {}
# [doc = " A trait that distinguishes some ChaCha variants. Contains configurations"] # [doc = " for \"Legacy\" DJB variant and the IETF variant."] pub trait Variant : sealed :: Sealed { # [doc = " The counter's type."] # [cfg (not (feature = "cipher"))] type Counter ; # [doc = " The counter's type."] # [cfg (feature = "cipher")] type Counter : cipher :: StreamCipherCounter ; # [doc = " Takes a slice of `state[12..NONCE_INDEX]` to convert it into"] # [doc = " `Self::Counter`."] fn get_block_pos (row : & [u32]) -> Self :: Counter ; # [doc = " Breaks down the `Self::Counter` type into a u32 array for setting the"] # [doc = " block pos."] fn set_block_pos (row : & mut [u32] , pos : Self :: Counter) ; # [doc = " A helper method for calculating the remaining blocks using these types"] fn remaining_blocks (block_pos : Self :: Counter) -> Option < usize > ; }
};
}
