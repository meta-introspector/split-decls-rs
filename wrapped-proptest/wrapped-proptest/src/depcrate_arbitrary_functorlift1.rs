// Generated macro for lift1 (macro)
macro_rules! Depcrate_arbitrary_functorlift1 {
() => {
// Module: crate::arbitrary::functor
// Provides: {"lift1"}
// Dependencies: {}
macro_rules ! lift1 { ([$ ($ bounds : tt) *] $ typ : ty , $ params : ty ; $ base : ident , $ args : ident => $ logic : expr) => { impl < A : :: core :: fmt :: Debug + $ ($ bounds) *> $ crate :: arbitrary :: functor :: ArbitraryF1 < A > for $ typ { type Parameters = $ params ; fn lift1_with < S > ($ base : S , $ args : Self :: Parameters) -> $ crate :: strategy :: BoxedStrategy < Self > where S : $ crate :: strategy :: Strategy < Value = A > + 'static { $ crate :: strategy :: Strategy :: boxed ($ logic) } } } ; ([$ ($ bounds : tt) *] $ typ : ty ; $ base : ident => $ logic : expr) => { lift1 ! ([$ ($ bounds) *] $ typ , () ; $ base , _args => $ logic) ; } ; ([$ ($ bounds : tt) *] $ typ : ty ; $ mapper : expr) => { lift1 ! (['static + $ ($ bounds) *] $ typ ; base => $ crate :: strategy :: Strategy :: prop_map (base , $ mapper)) ; } ; ([$ ($ bounds : tt) *] $ typ : ty) => { lift1 ! (['static + $ ($ bounds) *] $ typ ; base => $ crate :: strategy :: Strategy :: prop_map_into (base)) ; } ; }
};
}
