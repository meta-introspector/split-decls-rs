// Generated macro for float_edge_cases (macro)
macro_rules! Depcratefloat_edge_cases {
() => {
// Module: crate
// Provides: {"float_edge_cases"}
// Dependencies: {}
macro_rules ! float_edge_cases { ($ F : ident , $ case : ident , $ inner : block) => { for exponent in [F :: Int :: ZERO , F :: Int :: ONE , F :: Int :: ONE << (F :: EXP_BITS / 2) , (F :: Int :: ONE << (F :: EXP_BITS - 1)) - F :: Int :: ONE , F :: Int :: ONE << (F :: EXP_BITS - 1) , (F :: Int :: ONE << (F :: EXP_BITS - 1)) + F :: Int :: ONE , (F :: Int :: ONE << F :: EXP_BITS) - F :: Int :: ONE ,] . iter () { for significand in [F :: Int :: ZERO , F :: Int :: ONE , F :: Int :: ONE << (F :: SIG_BITS / 2) , (F :: Int :: ONE << (F :: SIG_BITS - 1)) - F :: Int :: ONE , F :: Int :: ONE << (F :: SIG_BITS - 1) , (F :: Int :: ONE << (F :: SIG_BITS - 1)) + F :: Int :: ONE , (F :: Int :: ONE << F :: SIG_BITS) - F :: Int :: ONE ,] . iter () { for sign in [false , true] . iter () { let $ case = F :: from_parts (* sign , * exponent , * significand) ; $ inner } } } } ; }
};
}
