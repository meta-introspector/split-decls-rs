// Generated macro for impl_101 (impl)
macro_rules! Depcrate_objectpathimpl_101 {
() => {
// Module: crate::objectpath
// Provides: {"impl_101"}
// Dependencies: {}
impl < M : MethodType < D > , D : DataType > IfaceCache < M , D > { pub fn get_factory < S : Into < IfaceName < 'static > > + Clone , F > (& self , s : S , f : F) -> Arc < Interface < M , D > > where F : FnOnce () -> Interface < M , D > { let s2 = s . clone () . into () ; let mut m = self . 0 . lock () . unwrap () ; m . entry (s2) . or_insert_with (| | { Arc :: new (f ()) }) . clone () } pub fn new () -> Arc < Self > { Arc :: new (IfaceCache (Mutex :: new (ArcMap :: new ()))) } }
};
}
