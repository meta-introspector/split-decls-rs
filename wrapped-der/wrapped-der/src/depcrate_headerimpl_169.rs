// Generated macro for impl_169 (impl)
macro_rules! Depcrate_headerimpl_169 {
() => {
// Module: crate::header
// Provides: {"impl_169"}
// Dependencies: {}
impl < 'a > Decode < 'a > for Header { type Error = Error ; fn decode < R : Reader < 'a > > (reader : & mut R) -> Result < Header > { let (tag , is_constructed) = Tag :: decode_with_constructed_bit (reader) ? ; let length = Length :: decode (reader) . map_err (| e | { if e . kind () == ErrorKind :: Overlength { reader . error (tag . length_error ()) } else { e } }) ? ; # [cfg (feature = "ber")] if length . is_indefinite () && ! is_constructed { debug_assert_eq ! (reader . encoding_rules () , EncodingRules :: Ber) ; return Err (reader . error (ErrorKind :: IndefiniteLength)) ; } # [cfg (not (feature = "ber"))] debug_assert_eq ! (is_constructed , tag . is_constructed ()) ; Ok (Self { tag , length , constructed : is_constructed , }) } }
};
}
