// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl < T > OptionIsSomeAndExt < T > for Option < T > { fn is_some_and (self , f : impl FnOnce (T) -> bool) -> bool { match self { None => false , Some (x) => f (x) , } } }
};
}
