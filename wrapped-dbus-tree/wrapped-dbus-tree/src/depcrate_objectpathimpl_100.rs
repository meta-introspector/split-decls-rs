// Generated macro for impl_100 (impl)
macro_rules! Depcrate_objectpathimpl_100 {
() => {
// Module: crate::objectpath
// Provides: {"impl_100"}
// Dependencies: {}
impl < M : MethodType < D > , D : DataType > IfaceCache < M , D > where D :: Interface : Default { pub fn get < S : Into < IfaceName < 'static > > + Clone , F > (& self , s : S , f : F) -> Arc < Interface < M , D > > where F : FnOnce (Interface < M , D >) -> Interface < M , D > { let s2 = s . clone () . into () ; let mut m = self . 0 . lock () . unwrap () ; m . entry (s2) . or_insert_with (| | { let i = new_interface (s . into () , Default :: default ()) ; Arc :: new (f (i)) }) . clone () } }
};
}
