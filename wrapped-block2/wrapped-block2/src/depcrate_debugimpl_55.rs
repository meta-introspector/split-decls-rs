// Generated macro for impl_55 (impl)
macro_rules! Depcrate_debugimpl_55 {
() => {
// Module: crate::debug
// Provides: {"impl_55"}
// Dependencies: {}
impl Debug for Isa { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { if self . is_global () { f . write_str ("_NSConcreteGlobalBlock") } else if self . is_stack () { f . write_str ("_NSConcreteStackBlock") } else { write ! (f , "{:?} (likely _NSConcreteMallocBlock)" , self . 0) } } }
};
}
