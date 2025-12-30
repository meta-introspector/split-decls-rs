// Generated macro for impl_84 (impl)
macro_rules! Depcrate_inputimpl_84 {
() => {
// Module: crate::input
// Provides: {"impl_84"}
// Dependencies: {}
impl fmt :: Display for CyclicDependenciesError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let render = | (id , name) : & (CrateBuilderId , Option < CrateDisplayName >) | match name { Some (it) => format ! ("{it}({id:?})") , None => format ! ("{id:?}") , } ; let path = self . path . iter () . rev () . map (render) . collect :: < Vec < String > > () . join (" -> ") ; write ! (f , "cyclic deps: {} -> {}, alternative path: {}" , render (self . from ()) , render (self . to ()) , path) } }
};
}
