// Generated macro for proto_ecn (function)
macro_rules! Depcrate_endpointproto_ecn {
() => {
// Module: crate::endpoint
// Provides: {"proto_ecn"}
// Dependencies: {}
# [inline] fn proto_ecn (ecn : udp :: EcnCodepoint) -> proto :: EcnCodepoint { match ecn { udp :: EcnCodepoint :: Ect0 => proto :: EcnCodepoint :: Ect0 , udp :: EcnCodepoint :: Ect1 => proto :: EcnCodepoint :: Ect1 , udp :: EcnCodepoint :: Ce => proto :: EcnCodepoint :: Ce , } }
};
}
