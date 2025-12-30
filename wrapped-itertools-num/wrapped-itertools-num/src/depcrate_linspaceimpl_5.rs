// Generated macro for impl_5 (impl)
macro_rules! Depcrate_linspaceimpl_5 {
() => {
// Module: crate::linspace
// Provides: {"impl_5"}
// Dependencies: {}
impl < F > Iterator for Linspace < F > where F : Float { type Item = F ; # [inline] fn next (& mut self) -> Option < F > { if self . index >= self . len { None } else { let i = self . index ; self . index += 1 ; Some (self . start + self . step * F :: from (i) . unwrap ()) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let n = self . len - self . index ; (n , Some (n)) } }
};
}
