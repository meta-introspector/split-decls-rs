// Generated macro for try_from_integers (macro)
macro_rules! Depcrate_types_idtry_from_integers {
() => {
// Module: crate::types::id
// Provides: {"try_from_integers"}
// Dependencies: {}
macro_rules ! try_from_integers { ($ ($ ty : ty) ,*) => { $ (impl TryFrom < ID > for $ ty { type Error = ParseIntError ; fn try_from (id : ID) -> Result < Self , Self :: Error > { id . 0 . parse () } }) * } ; }
};
}
