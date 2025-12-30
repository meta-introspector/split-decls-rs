// Generated macro for impl_132 (impl)
macro_rules! Depcrate_machinst_vcodeimpl_132 {
() => {
// Module: crate::machinst::vcode
// Provides: {"impl_132"}
// Dependencies: {}
impl < I : VCodeInst > Debug for VRegAllocator < I > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { writeln ! (f , "VRegAllocator {{") ? ; let mut alias_keys = self . vreg_aliases . keys () . cloned () . collect :: < Vec < _ > > () ; alias_keys . sort_unstable () ; for key in alias_keys { let dest = self . vreg_aliases . get (& key) . unwrap () ; writeln ! (f , "  {:?} := {:?}" , Reg :: from (key) , Reg :: from (* dest)) ? ; } for (vreg , fact) in self . facts . iter () . enumerate () { if let Some (fact) = fact { writeln ! (f , "  v{vreg} ! {fact}") ? ; } } writeln ! (f , "}}") } }
};
}
