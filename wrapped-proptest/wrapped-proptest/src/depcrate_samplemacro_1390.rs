// Generated macro for macro_1390 (macro)
macro_rules! Depcrate_samplemacro_1390 {
() => {
// Module: crate::sample
// Provides: {"macro_1390"}
// Dependencies: {}
opaque_strategy_wrapper ! { # [doc = " Strategy to produce one value from a fixed collection of options."] # [doc = ""] # [doc = " Created by the `select()` in the same module."] # [derive (Clone , Debug)] pub struct Select [< T >] [where T : Clone + fmt :: Debug + 'static] (statics :: Map < Range < usize >, SelectMapFn < T >>) -> SelectValueTree < T >; # [doc = " `ValueTree` corresponding to `Select`."] # [derive (Clone , Debug)] pub struct SelectValueTree [< T >] [where T : Clone + fmt :: Debug + 'static] (statics :: Map < num :: usize :: BinarySearch , SelectMapFn < T >>) -> T ; }
};
}
