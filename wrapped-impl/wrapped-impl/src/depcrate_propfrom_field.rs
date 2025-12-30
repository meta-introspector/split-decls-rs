// Generated macro for from_field (function)
macro_rules! Depcrate_propfrom_field {
() => {
// Module: crate::prop
// Provides: {"from_field"}
// Dependencies: {}
fn from_field < 'a , 'b > (fields : & 'a [Field < 'b >]) -> Option < & 'a Field < 'b > > { for field in fields { if field . attrs . from . is_some () { return Some (field) ; } } None }
};
}
