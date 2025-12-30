// Generated macro for prim (macro)
macro_rules! Depcrate_impls_primitivesprim {
() => {
// Module: crate::impls::primitives
// Provides: {"prim"}
// Dependencies: {}
macro_rules ! prim { ($ ty : ty , $ fmt : literal , $ self_ : ident , $ write : expr) => { impl Format for $ ty { default_format ! () ; # [inline] fn _format_tag () -> Str { internp ! ($ fmt) } # [inline] fn _format_data (&$ self_) { $ write } } } ; }
};
}
