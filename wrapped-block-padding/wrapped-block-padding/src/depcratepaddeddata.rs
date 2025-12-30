// Generated macro for PaddedData (enum)
macro_rules! DepcratePaddedData {
() => {
// Module: crate
// Provides: {"PaddedData"}
// Dependencies: {}
# [doc = " Padded data split into blocks with detached last block returned by [`Padding::pad_detached`]."] # [derive (Debug)] pub enum PaddedData < 'a , BlockSize : ArraySize > { # [doc = " Message split into blocks with detached and padded `tail_block`."] Pad { # [doc = " Message blocks."] blocks : & 'a [Array < u8 , BlockSize >] , # [doc = " Last message block with padding."] tail_block : Array < u8 , BlockSize > , } , # [doc = " [`NoPadding`] or [`ZeroPadding`] were used on a message which does not require any padding."] NoPad { # [doc = " Message blocks."] blocks : & 'a [Array < u8 , BlockSize >] , } , # [doc = " [`NoPadding`] was used on a message with size not multiple of the block size."] Error , }
};
}
