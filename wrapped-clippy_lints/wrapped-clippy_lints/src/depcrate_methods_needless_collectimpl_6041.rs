// Generated macro for impl_6041 (impl)
macro_rules! Depcrate_methods_needless_collectimpl_6041 {
() => {
// Module: crate::methods::needless_collect
// Provides: {"impl_6041"}
// Dependencies: {}
impl IterFunction { fn get_iter_method (& self , cx : & LateContext < '_ >) -> String { match & self . func { IterFunctionKind :: IntoIter (_) => String :: new () , IterFunctionKind :: Len => String :: from (".count()") , IterFunctionKind :: IsEmpty => String :: from (".next().is_none()") , IterFunctionKind :: Contains (span) => { let s = snippet (cx , * span , "..") ; if let Some (stripped) = s . strip_prefix ('&') { format ! (".any(|x| x == {stripped})") } else { format ! (".any(|x| x == *{s})") } } , } } fn get_suggestion_text (& self) -> & 'static str { match & self . func { IterFunctionKind :: IntoIter (_) => { "use the original Iterator instead of collecting it and then producing a new one" } , IterFunctionKind :: Len => { "take the original Iterator's count instead of collecting it and finding the length" } , IterFunctionKind :: IsEmpty => { "check if the original Iterator has anything instead of collecting it and seeing if it's empty" } , IterFunctionKind :: Contains (_) => { "check if the original Iterator contains an element instead of collecting then checking" } , } } }
};
}
