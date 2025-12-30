// Generated macro for arbitrary (macro)
macro_rules! Depcrate_arbitrary_macrosarbitrary {
() => {
// Module: crate::arbitrary::macros
// Provides: {"arbitrary"}
// Dependencies: {}
macro_rules ! arbitrary { ([$ ($ bounds : tt) *] $ typ : ty , $ strat : ty , $ params : ty ; $ args : ident => $ logic : expr) => { impl <$ ($ bounds) *> $ crate :: arbitrary :: Arbitrary for $ typ { type Parameters = $ params ; type Strategy = $ strat ; fn arbitrary_with ($ args : Self :: Parameters) -> Self :: Strategy { $ logic } } } ; ([$ ($ bounds : tt) *] $ typ : ty , $ strat : ty ; $ logic : expr) => { arbitrary ! ([$ ($ bounds) *] $ typ , $ strat , () ; _args => $ logic) ; } ; ([$ ($ bounds : tt) *] $ typ : ty ; $ logic : expr) => { arbitrary ! ([$ ($ bounds) *] $ typ , $ crate :: strategy :: Just < Self >, () ; _args => $ crate :: strategy :: Just ($ logic)) ; } ; ($ typ : ty , $ strat : ty , $ params : ty ; $ args : ident => $ logic : expr) => { arbitrary ! ([] $ typ , $ strat , $ params ; $ args => $ logic) ; } ; ($ typ : ty , $ strat : ty ; $ logic : expr) => { arbitrary ! ([] $ typ , $ strat ; $ logic) ; } ; ($ strat : ty ; $ logic : expr) => { arbitrary ! ([] $ strat ; $ logic) ; } ; ($ ($ typ : ident) ,*) => { $ (arbitrary ! ($ typ , $ typ :: Any ; $ typ :: ANY) ;) * } ; }
};
}
