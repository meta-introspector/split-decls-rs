// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'a > DecodeValue < 'a > for InitialContextToken < 'a > { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , _header : der :: Header) -> der :: Result < Self > { Ok (Self { this_mech : reader . decode () ? , inner_context_token : reader . decode () ? , }) } }
};
}
