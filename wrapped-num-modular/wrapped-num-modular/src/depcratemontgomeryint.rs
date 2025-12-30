// Generated macro for MontgomeryInt (type)
macro_rules! DepcrateMontgomeryInt {
() => {
// Module: crate
// Provides: {"MontgomeryInt"}
// Dependencies: {}
# [doc = " An integer in modulo ring based on [Montgomery form](https://en.wikipedia.org/wiki/Montgomery_modular_multiplication#Montgomery_form)"] pub type MontgomeryInt < T > = ReducedInt < T , Montgomery < T > > ;
};
}
