// Generated macro for impl_103 (impl)
macro_rules! Depcrate_decodeimpl_103 {
() => {
// Module: crate::decode
// Provides: {"impl_103"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a , T , E > DecodeValue < 'a > for Cow < 'a , T > where T : ToOwned + ? Sized , & 'a T : DecodeValue < 'a , Error = E > , T :: Owned : for < 'b > DecodeValue < 'b , Error = E > , E : From < Error > + 'static , { type Error = E ; fn decode_value < R : Reader < 'a > > (reader : & mut R , header : Header) -> Result < Self , Self :: Error > { if R :: CAN_READ_SLICE { < & 'a T > :: decode_value (reader , header) . map (Cow :: Borrowed) } else { T :: Owned :: decode_value (reader , header) . map (Cow :: Owned) } } }
};
}
