// Generated macro for wrap_from (macro)
macro_rules! Depcrate_arbitrary_macroswrap_from {
() => {
// Module: crate::arbitrary::macros
// Provides: {"wrap_from"}
// Dependencies: {}
macro_rules ! wrap_from { ($ wrap : ident) => { wrap_from ! ([] $ wrap) ; } ; ([$ ($ bound : tt) *] $ wrap : ident) => { arbitrary ! ([A : $ crate :: arbitrary :: Arbitrary + $ ($ bound) *] $ wrap < A >, $ crate :: strategy :: MapInto < A :: Strategy , Self >, A :: Parameters ; args => $ crate :: strategy :: Strategy :: prop_map_into ($ crate :: arbitrary :: any_with ::< A > (args))) ; lift1 ! ([$ ($ bound) *] $ wrap < A >) ; } ; }
};
}
