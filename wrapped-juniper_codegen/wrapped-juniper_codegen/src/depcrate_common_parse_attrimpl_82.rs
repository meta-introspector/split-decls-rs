// Generated macro for impl_82 (impl)
macro_rules! Depcrate_common_parse_attrimpl_82 {
() => {
// Module: crate::common::parse::attr
// Provides: {"impl_82"}
// Dependencies: {}
impl < T > OptionExt for Option < T > { type Inner = T ; fn none_or_else < E , F > (self , err : F) -> Result < () , E > where F : FnOnce (T) -> E , { match self { Some (v) => Err (err (v)) , None => Ok (()) , } } }
};
}
