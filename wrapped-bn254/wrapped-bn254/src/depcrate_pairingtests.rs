// Generated macro for tests (module)
macro_rules! Depcrate_pairingtests {
() => {
// Module: crate::pairing
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn alt_bn128_pairing_invalid_length () { let input = [0 ; 193] ; let result = alt_bn128_pairing_be (& input) ; assert ! (result . is_err ()) ; } }
};
}
