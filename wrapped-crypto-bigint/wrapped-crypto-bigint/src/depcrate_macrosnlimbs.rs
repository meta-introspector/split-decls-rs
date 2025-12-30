// Generated macro for nlimbs (macro)
macro_rules! Depcrate_macrosnlimbs {
() => {
// Module: crate::macros
// Provides: {"nlimbs"}
// Dependencies: {}
# [doc = " Calculate the number of limbs required to represent the given number of bits."] # [macro_export] macro_rules ! nlimbs { ($ bits : expr) => { u32 :: div_ceil ($ bits , $ crate :: Limb :: BITS) as usize } ; }
};
}
