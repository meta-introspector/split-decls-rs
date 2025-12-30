// Generated macro for wrap_ctor (macro)
macro_rules! Depcrate_arbitrary_macroswrap_ctor {
() => {
// Module: crate::arbitrary::macros
// Provides: {"wrap_ctor"}
// Dependencies: {}
macro_rules ! wrap_ctor { ($ wrap : ident) => { wrap_ctor ! ([] $ wrap) ; } ; ($ wrap : ident , $ maker : expr) => { wrap_ctor ! ([] $ wrap , $ maker) ; } ; ([$ ($ bound : tt) *] $ wrap : ident) => { wrap_ctor ! ([$ ($ bound) *] $ wrap , $ wrap :: new) ; } ; ([$ ($ bound : tt) *] $ wrap : ident , $ maker : expr) => { arbitrary ! ([A : $ crate :: arbitrary :: Arbitrary + $ ($ bound) *] $ wrap < A >, $ crate :: arbitrary :: SMapped < A , Self >, A :: Parameters ; args => $ crate :: strategy :: statics :: static_map ($ crate :: arbitrary :: any_with ::< A > (args) , $ maker)) ; lift1 ! ([$ ($ bound) *] $ wrap < A >; $ maker) ; } ; }
};
}
