// Generated macro for impl_43 (impl)
macro_rules! Depcrate_bsslimpl_43 {
() => {
// Module: crate::bssl
// Provides: {"impl_43"}
// Dependencies: {}
impl From < Result > for core :: result :: Result < () , error :: Unspecified > { fn from (ret : Result) -> Self { match ret . 0 { 1 => Ok (()) , c => { debug_assert_eq ! (c , 0 , "`bssl::Result` value must be 0 or 1") ; Err (error :: Unspecified) } } } }
};
}
