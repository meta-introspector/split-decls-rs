// Generated macro for impl_22 (impl)
macro_rules! Depcrate_decodeimpl_22 {
() => {
// Module: crate::decode
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'src , T : Decode < 'src > > Decode < 'src > for Option < T > { fn decode (data : & mut & 'src [u8]) -> Self { match get (data) { 0 => None , 1 => Some (Decode :: decode (data)) , _ => unreachable ! () , } } }
};
}
