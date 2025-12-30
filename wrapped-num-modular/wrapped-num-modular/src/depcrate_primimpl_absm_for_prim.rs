// Generated macro for impl_absm_for_prim (macro)
macro_rules! Depcrate_primimpl_absm_for_prim {
() => {
// Module: crate::prim
// Provides: {"impl_absm_for_prim"}
// Dependencies: {}
macro_rules ! impl_absm_for_prim { ($ ($ signed : ty => $ unsigned : ty ;) *) => { $ (impl ModularAbs <$ unsigned > for $ signed { fn absm (self , m : &$ unsigned) -> $ unsigned { if self >= 0 { (self as $ unsigned) % m } else { (- self as $ unsigned) . negm (m) } } }) * } ; }
};
}
