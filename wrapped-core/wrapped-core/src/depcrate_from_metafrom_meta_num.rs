// Generated macro for from_meta_num (macro)
macro_rules! Depcrate_from_metafrom_meta_num {
() => {
// Module: crate::from_meta
// Provides: {"from_meta_num"}
// Dependencies: {}
# [doc = " Generate an impl of `FromMeta` that will accept strings which parse to numbers or"] # [doc = " integer literals."] macro_rules ! from_meta_num { ($ ty : path) => { impl FromMeta for $ ty { fn from_string (s : & str) -> Result < Self > { s . parse () . map_err (| _ | Error :: unknown_value (s)) } fn from_value (value : & Lit) -> Result < Self > { (match * value { Lit :: Str (ref s) => Self :: from_string (& s . value ()) , Lit :: Int (ref s) => s . base10_parse ::<$ ty > () . map_err (Error :: from) , _ => Err (Error :: unexpected_lit_type (value)) , }) . map_err (| e | e . with_span (value)) } } } ; }
};
}
