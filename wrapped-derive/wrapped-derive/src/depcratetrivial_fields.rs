// Generated macro for trivial_fields (function)
macro_rules! Depcratetrivial_fields {
() => {
// Module: crate
// Provides: {"trivial_fields"}
// Dependencies: {}
fn trivial_fields (fields : & Fields) -> Result < Vec < & Type > > { let is_trivial = decide_trivial (fields) ? ; let mut trivial = Vec :: new () ; for field in fields { if is_trivial (field) ? { trivial . push (& field . ty) ; } } Ok (trivial) }
};
}
