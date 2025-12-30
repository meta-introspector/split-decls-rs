// Generated macro for impl_387 (impl)
macro_rules! Depcrate_hirimpl_387 {
() => {
// Module: crate::hir
// Provides: {"impl_387"}
// Dependencies: {}
impl Defaultness { pub fn has_value (& self) -> bool { match * self { Defaultness :: Default { has_value } => has_value , Defaultness :: Final => true , } } pub fn is_final (& self) -> bool { * self == Defaultness :: Final } pub fn is_default (& self) -> bool { matches ! (* self , Defaultness :: Default { .. }) } }
};
}
