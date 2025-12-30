// Generated macro for impl_1498 (impl)
macro_rules! Depcrate_validatorsimpl_1498 {
() => {
// Module: crate::validators
// Provides: {"impl_1498"}
// Dependencies: {}
impl < T , F , E > CustomValidator < T > for F where T : InputType , E : Into < InputValueError < T > > , F : Fn (& T) -> Result < () , E > , { # [inline] fn check (& self , value : & T) -> Result < () , InputValueError < T > > { (self) (value) . map_err (Into :: into) } }
};
}
