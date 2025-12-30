// Generated macro for GeneratorObj (struct)
macro_rules! Depcrate_gen_implGeneratorObj {
() => {
// Module: crate::gen_impl
// Provides: {"GeneratorObj"}
// Dependencies: {}
# [doc = " the generator obj type, the functor passed to it must be Send"] pub struct GeneratorObj < 'a , A , T , const LOCAL : bool > { gen : StackBox < GeneratorImpl < 'a , A , T > > , }
};
}
