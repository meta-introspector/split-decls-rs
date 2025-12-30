// Generated macro for impl_float_const (macro)
macro_rules! Depcrateimpl_float_const {
() => {
// Module: crate
// Provides: {"impl_float_const"}
// Dependencies: {}
macro_rules ! impl_float_const { ($ type : ident , $ wrapper : expr) => { impl < T : FloatConst > FloatConst for $ type < T > { impl_float_const_method ! ($ wrapper , E) ; impl_float_const_method ! ($ wrapper , FRAC_1_PI) ; impl_float_const_method ! ($ wrapper , FRAC_1_SQRT_2) ; impl_float_const_method ! ($ wrapper , FRAC_2_PI) ; impl_float_const_method ! ($ wrapper , FRAC_2_SQRT_PI) ; impl_float_const_method ! ($ wrapper , FRAC_PI_2) ; impl_float_const_method ! ($ wrapper , FRAC_PI_3) ; impl_float_const_method ! ($ wrapper , FRAC_PI_4) ; impl_float_const_method ! ($ wrapper , FRAC_PI_6) ; impl_float_const_method ! ($ wrapper , FRAC_PI_8) ; impl_float_const_method ! ($ wrapper , LN_10) ; impl_float_const_method ! ($ wrapper , LN_2) ; impl_float_const_method ! ($ wrapper , LOG10_E) ; impl_float_const_method ! ($ wrapper , LOG2_E) ; impl_float_const_method ! ($ wrapper , PI) ; impl_float_const_method ! ($ wrapper , SQRT_2) ; } } ; }
};
}
