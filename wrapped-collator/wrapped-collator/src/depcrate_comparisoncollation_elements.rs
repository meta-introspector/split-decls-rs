// Generated macro for collation_elements (macro)
macro_rules! Depcrate_comparisoncollation_elements {
() => {
// Module: crate::comparison
// Provides: {"collation_elements"}
// Dependencies: {}
macro_rules ! collation_elements { ($ self : expr , $ chars : expr , $ tailoring : expr , $ numeric_primary : expr) => { { let jamo = <& [< u32 as AsULE >:: ULE ; JAMO_COUNT] >:: try_from ($ self . jamo . ce32s . as_ule_slice ()) ; let jamo = jamo . unwrap () ; CollationElements :: new ($ chars , $ self . root , $ tailoring , jamo , &$ self . diacritics . secondaries , $ self . decompositions , $ self . tables , $ numeric_primary , $ self . lithuanian_dot_above ,) } } ; }
};
}
