// Generated macro for tests (module)
macro_rules! Depcrate_serdetests {
() => {
// Module: crate::serde
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { Signature , SignatureBytes } ; use hex_literal :: hex ; const SIGNATURE_BYTES : SignatureBytes = hex ! ("
        e5564300c360ac729086e2cc806e828a
        84877f1eb8e5d974d873e06522490155
        5fb8821590a33bacc61e39701cf9b46b
        d25bf5f0595bbe24655141438e7a100b
        ") ; # [test] fn round_trip () { let signature = Signature :: from_bytes (& SIGNATURE_BYTES) ; let serialized = bincode :: serialize (& signature) . expect ("serialized") ; let deserialized = bincode :: deserialize (& serialized) . expect ("deserialized") ; assert_eq ! (signature , deserialized) ; } }
};
}
