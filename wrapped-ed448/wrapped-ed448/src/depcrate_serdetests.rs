// Generated macro for tests (module)
macro_rules! Depcrate_serdetests {
() => {
// Module: crate::serde
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { Signature , SignatureBytes } ; use hex_literal :: hex ; const SIGNATURE_BYTES : SignatureBytes = hex ! ("533a37f6bbe457251f023c0d88f976ae
        2dfb504a843e34d2074fd823d41a591f
        2b233f034f628281f2fd7a22ddd47d78
        28c59bd0a21bfd3980ff0d2028d4b18a
        9df63e006c5d1c2d345b925d8dc00b41
        04852db99ac5c7cdda8530a113a0f4db
        b61149f05a7363268c71d95808ff2e65
        2600") ; # [test] fn round_trip () { let signature = Signature :: from_bytes (& SIGNATURE_BYTES) ; let serialized = bincode :: serialize (& signature) . expect ("serialized") ; let deserialized = bincode :: deserialize (& serialized) . expect ("deserialized") ; assert_eq ! (signature , deserialized) ; } }
};
}
