// Generated macro for udp_ecn (function)
macro_rules! Depcrateudp_ecn {
() => {
// Module: crate
// Provides: {"udp_ecn"}
// Dependencies: {}
fn udp_ecn (ecn : proto :: EcnCodepoint) -> udp :: EcnCodepoint { match ecn { proto :: EcnCodepoint :: Ect0 => udp :: EcnCodepoint :: Ect0 , proto :: EcnCodepoint :: Ect1 => udp :: EcnCodepoint :: Ect1 , proto :: EcnCodepoint :: Ce => udp :: EcnCodepoint :: Ce , } }
};
}
