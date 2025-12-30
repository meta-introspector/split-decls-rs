// Generated macro for as_ref_slice (function)
macro_rules! Depcrate_converteras_ref_slice {
() => {
// Module: crate::converter
// Provides: {"as_ref_slice"}
// Dependencies: {}
# [test] fn as_ref_slice () { use std :: collections :: HashSet ; let mut set = IteratorAsRefSlice :: < HashSet < usize > , usize > :: default () ; set . insert (42) ; assert_eq ! (set . bake (& Default :: default ()) . to_string () , r#"& [42usize ,]"#) }
};
}
