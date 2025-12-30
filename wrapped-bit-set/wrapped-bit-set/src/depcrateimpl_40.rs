// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
impl < B : BitBlock > Iterator for Intersection < '_ , B > { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { if self . n != 0 { self . n -= 1 ; self . iter . next () } else { None } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (self . n)) } # [inline] fn count (self) -> usize { self . iter . count () } }
};
}
