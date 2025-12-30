// Generated macro for macro_198 (macro)
macro_rules! Depcrate_errormacro_198 {
() => {
// Module: crate::error
// Provides: {"macro_198"}
// Dependencies: {}
fatal ! (cant_set_param_but_not_strat (self_ty : & syn :: Type , item : & str) , E0011 , "Cannot set `#[proptest(params = <type>)]` on {0} while not providing a \
     strategy for the {0} to use it since `<{1} as Arbitrary<'a>>::Strategy` \
     may require a different type than the one provided in `<type>`." , item , quote ! { # self_ty }) ;
};
}
