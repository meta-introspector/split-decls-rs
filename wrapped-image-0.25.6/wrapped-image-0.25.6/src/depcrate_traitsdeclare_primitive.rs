// Generated macro for declare_primitive (macro)
macro_rules! Depcrate_traitsdeclare_primitive {
() => {
// Module: crate::traits
// Provides: {"declare_primitive"}
// Dependencies: {}
macro_rules ! declare_primitive { ($ base : ty : ($ from : expr) ..$ to : expr) => { impl Primitive for $ base { const DEFAULT_MAX_VALUE : Self = $ to ; const DEFAULT_MIN_VALUE : Self = $ from ; } } ; }
};
}
