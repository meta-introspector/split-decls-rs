// Generated macro for impl_235 (impl)
macro_rules! Depcrate_responseimpl_235 {
() => {
// Module: crate::response
// Provides: {"impl_235"}
// Dependencies: {}
impl DataPayload < BufferMarker > { # [doc = " Converts an owned byte buffer into a `DataPayload<BufferMarker>`."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] pub fn from_owned_buffer (buffer : Box < [u8] >) -> Self { let yoke = Yoke :: attach_to_cart (SelectedRc :: new (buffer) , | b | & * * b) . wrap_cart_in_option () . convert_cart_into_option_pointer () ; Self (DataPayloadInner :: Yoke (yoke)) } # [doc = " Converts a yoked byte buffer into a `DataPayload<BufferMarker>`."] pub fn from_yoked_buffer (yoke : Yoke < & 'static [u8] , Option < Cart > >) -> Self { let yoke = Cart :: unwrap_cart (yoke) ; Self (DataPayloadInner :: Yoke (yoke . convert_cart_into_option_pointer () ,)) } # [doc = " Converts a static byte buffer into a `DataPayload<BufferMarker>`."] pub fn from_static_buffer (buffer : & 'static [u8]) -> Self { Self (DataPayloadInner :: Yoke (Yoke :: new_owned (buffer) . convert_cart_into_option_pointer () ,)) } }
};
}
