// Generated macro for impl_30 (impl)
macro_rules! Depcrate_pbes1impl_30 {
() => {
// Module: crate::pbes1
// Provides: {"impl_30"}
// Dependencies: {}
impl TryFrom < AnyRef < '_ > > for Parameters { type Error = der :: Error ; fn try_from (any : AnyRef < '_ >) -> der :: Result < Parameters > { any . sequence (| reader | { Ok (Parameters { salt : < & OctetStringRef > :: decode (reader) ? . as_bytes () . try_into () . map_err (| _ | Tag :: OctetString . value_error ()) ? , iteration_count : reader . decode () ? , }) }) } }
};
}
