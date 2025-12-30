// Generated macro for smart_pointer_t (macro)
macro_rules! Depcrate_from_metasmart_pointer_t {
() => {
// Module: crate::from_meta
// Provides: {"smart_pointer_t"}
// Dependencies: {}
# [doc = " Create an impl that forwards to an inner type `T` for parsing."] macro_rules ! smart_pointer_t { ($ ty : path , $ map_fn : path) => { impl < T : FromMeta > FromMeta for $ ty { fn from_none () -> Option < Self > { T :: from_none () . map ($ map_fn) } fn from_list (items : & [NestedMeta]) -> Result < Self > { FromMeta :: from_list (items) . map ($ map_fn) } fn from_meta (item : & Meta) -> Result < Self > { FromMeta :: from_meta (item) . map ($ map_fn) } } } ; }
};
}
