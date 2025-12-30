// Generated macro for type_is_pointer (function)
macro_rules! Depcrate_commontype_is_pointer {
() => {
// Module: crate::common
// Provides: {"type_is_pointer"}
// Dependencies: {}
pub fn type_is_pointer (typ : Type < '_ >) -> bool { typ . get_pointee () . is_some () }
};
}
