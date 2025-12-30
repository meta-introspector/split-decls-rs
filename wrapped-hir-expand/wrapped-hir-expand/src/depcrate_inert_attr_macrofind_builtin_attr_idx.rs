// Generated macro for find_builtin_attr_idx (function)
macro_rules! Depcrate_inert_attr_macrofind_builtin_attr_idx {
() => {
// Module: crate::inert_attr_macro
// Provides: {"find_builtin_attr_idx"}
// Dependencies: {}
pub fn find_builtin_attr_idx (name : & Symbol) -> Option < usize > { static BUILTIN_LOOKUP_TABLE : OnceLock < FxHashMap < Symbol , usize > > = OnceLock :: new () ; BUILTIN_LOOKUP_TABLE . get_or_init (| | { INERT_ATTRIBUTES . iter () . map (| attr | attr . name) . enumerate () . map (| (a , b) | (Symbol :: intern (b) , a)) . collect () }) . get (name) . copied () }
};
}
