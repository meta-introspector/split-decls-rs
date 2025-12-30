// Generated macro for impl_58 (impl)
macro_rules! Depcrate_paramsimpl_58 {
() => {
// Module: crate::params
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'a > FromIterator < Pair < 'a > > for ParamsString { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = Pair < 'a > > , { let mut params = ParamsString :: new () ; for pair in iter { params . add_str (pair . 0 , pair . 1) . expect ("PHC params error") ; } params } }
};
}
