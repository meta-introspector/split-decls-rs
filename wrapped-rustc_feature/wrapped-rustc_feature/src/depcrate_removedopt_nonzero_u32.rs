// Generated macro for opt_nonzero_u32 (macro)
macro_rules! Depcrate_removedopt_nonzero_u32 {
() => {
// Module: crate::removed
// Provides: {"opt_nonzero_u32"}
// Dependencies: {}
macro_rules ! opt_nonzero_u32 { () => { None } ; ($ val : expr) => { Some (NonZeroU32 :: new ($ val) . unwrap ()) } ; }
};
}
