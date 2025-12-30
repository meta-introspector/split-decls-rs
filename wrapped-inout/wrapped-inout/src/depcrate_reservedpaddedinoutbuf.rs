// Generated macro for PaddedInOutBuf (struct)
macro_rules! Depcrate_reservedPaddedInOutBuf {
() => {
// Module: crate::reserved
// Provides: {"PaddedInOutBuf"}
// Dependencies: {}
# [doc = " Variant of [`InOutBuf`] with optional padded tail block."] # [cfg (feature = "block-padding")] # [allow (clippy :: type_complexity)] pub struct PaddedInOutBuf < 'inp , 'out , BS : ArraySize > { blocks : InOutBuf < 'inp , 'out , Array < u8 , BS > > , tail_inout : Option < (Array < u8 , BS > , & 'out mut Array < u8 , BS >) > , }
};
}
