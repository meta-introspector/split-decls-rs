// Generated macro for impl_29 (impl)
macro_rules! Depcrate_generate_componentsimpl_29 {
() => {
// Module: crate::generate::components
// Provides: {"impl_29"}
// Dependencies: {}
impl < 'a > DecodeValue < 'a > for Components { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , _header : Header) -> der :: Result < Self > { let p = reader . decode :: < UintRef < '_ > > () ? ; let q = reader . decode :: < UintRef < '_ > > () ? ; let g = reader . decode :: < UintRef < '_ > > () ? ; let p = BoxedUint :: from_be_slice_vartime (p . as_bytes ()) ; let q = BoxedUint :: from_be_slice_vartime (q . as_bytes ()) ; let g = BoxedUint :: from_be_slice_vartime (g . as_bytes ()) ; Self :: from_components (p , q , g) . map_err (| _ | reader . error (Tag :: Integer . value_error ())) } }
};
}
