// Generated macro for pae (function)
macro_rules! Depcrate_paepae {
() => {
// Module: crate::pae
// Provides: {"pae"}
// Dependencies: {}
# [doc = " Pre-Authentication Encoding. See [specification](https://github.com/paragonie/paseto/blob/master/docs/01-Protocol-Versions/Common.md#pae-definition)."] pub fn pae (pieces : & [& [u8]]) -> Result < Vec < u8 > , Error > { let mut out : Vec < u8 > = Vec :: with_capacity (64) ; out . extend_from_slice (& le64 (pieces . len () . try_into () ?)) ; for elem in pieces . iter () { out . extend_from_slice (& le64 (elem . len () . try_into () ?)) ; out . extend_from_slice (elem) ; } Ok (out) }
};
}
