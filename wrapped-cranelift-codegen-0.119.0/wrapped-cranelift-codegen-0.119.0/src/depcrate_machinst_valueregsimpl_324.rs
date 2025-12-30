// Generated macro for impl_324 (impl)
macro_rules! Depcrate_machinst_valueregsimpl_324 {
() => {
// Module: crate::machinst::valueregs
// Provides: {"impl_324"}
// Dependencies: {}
impl < R : Clone + Copy + Debug + PartialEq + Eq + InvalidSentinel > ValueRegs < R > { # [doc = " Create a Value-in-R location for a value stored in one register."] pub fn one (reg : R) -> Self { ValueRegs { parts : [reg , R :: invalid_sentinel ()] , } } # [doc = " Create a Value-in-R location for a value stored in two registers."] pub fn two (r1 : R , r2 : R) -> Self { ValueRegs { parts : [r1 , r2] } } # [doc = " Return the number of registers used."] pub fn len (self) -> usize { (self . parts [0] != R :: invalid_sentinel ()) as usize + (self . parts [1] != R :: invalid_sentinel ()) as usize } # [doc = " Map individual registers via a map function."] pub fn map < NewR , F > (self , f : F) -> ValueRegs < NewR > where NewR : Clone + Copy + Debug + PartialEq + Eq + InvalidSentinel , F : Fn (R) -> NewR , { ValueRegs { parts : [f (self . parts [0]) , f (self . parts [1])] , } } }
};
}
