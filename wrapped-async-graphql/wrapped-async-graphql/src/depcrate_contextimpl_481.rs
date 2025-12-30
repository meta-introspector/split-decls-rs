// Generated macro for impl_481 (impl)
macro_rules! Depcrate_contextimpl_481 {
() => {
// Module: crate::context
// Provides: {"impl_481"}
// Dependencies: {}
impl Display for QueryPathNode < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { let mut first = true ; self . try_for_each (| segment | { if ! first { write ! (f , ".") ? ; } first = false ; match segment { QueryPathSegment :: Index (idx) => write ! (f , "{}" , * idx) , QueryPathSegment :: Name (name) => write ! (f , "{}" , name) , } }) } }
};
}
