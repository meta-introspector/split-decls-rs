// Generated macro for impl_89 (impl)
macro_rules! Depcrate_astimpl_89 {
() => {
// Module: crate::ast
// Provides: {"impl_89"}
// Dependencies: {}
# [cfg (feature = "display")] impl Display for Port { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { match self { Port :: ID (name , Some (cpss)) => { write ! (f , ": {name} : {cpss}") } Port :: ID (name , None) => { write ! (f , ": {name}") } Port :: Compass (cpss) => { write ! (f , ": {}" , cpss) } } } }
};
}
