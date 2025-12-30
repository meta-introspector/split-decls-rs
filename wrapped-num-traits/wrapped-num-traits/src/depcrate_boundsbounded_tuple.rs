// Generated macro for bounded_tuple (macro)
macro_rules! Depcrate_boundsbounded_tuple {
() => {
// Module: crate::bounds
// Provides: {"bounded_tuple"}
// Dependencies: {}
macro_rules ! bounded_tuple { ($ ($ name : ident) *) => (impl <$ ($ name : Bounded ,) *> Bounded for ($ ($ name ,) *) { # [inline] fn min_value () -> Self { ($ ($ name :: min_value () ,) *) } # [inline] fn max_value () -> Self { ($ ($ name :: max_value () ,) *) } }) ; }
};
}
