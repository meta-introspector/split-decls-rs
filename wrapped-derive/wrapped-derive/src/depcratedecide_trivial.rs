// Generated macro for decide_trivial (function)
macro_rules! Depcratedecide_trivial {
() => {
// Module: crate
// Provides: {"decide_trivial"}
// Dependencies: {}
fn decide_trivial (fields : & Fields) -> Result < fn (& Field) -> Result < bool > > { for field in fields { if is_explicit_trivial (field) ? { return Ok (is_explicit_trivial) ; } } Ok (is_implicit_trivial) }
};
}
