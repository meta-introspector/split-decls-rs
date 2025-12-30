// Generated macro for s (macro)
macro_rules! Depcrate_macross {
() => {
// Module: crate::macros
// Provides: {"s"}
// Dependencies: {}
# [doc = " Implement `Clone` and `Copy` for a struct, as well as `Debug`, `Eq`, `Hash`, and"] # [doc = " `PartialEq` if the `extra_traits` feature is enabled."] # [doc = ""] # [doc = " Use [`s_no_extra_traits`] for structs where the `extra_traits` feature does not"] # [doc = " make sense, and for unions."] macro_rules ! s { ($ ($ (# [$ attr : meta]) * pub $ t : ident $ i : ident { $ ($ field : tt) * }) *) => ($ (s ! (it : $ (# [$ attr]) * pub $ t $ i { $ ($ field) * }) ;) *) ; (it : $ (# [$ attr : meta]) * pub union $ i : ident { $ ($ field : tt) * }) => (compile_error ! ("unions cannot derive extra traits, use s_no_extra_traits instead") ;) ; (it : $ (# [$ attr : meta]) * pub struct $ i : ident { $ ($ field : tt) * }) => (__item ! { # [repr (C)] # [cfg_attr (feature = "extra_traits" , :: core :: prelude :: v1 :: derive (Debug , Eq , Hash , PartialEq))] # [:: core :: prelude :: v1 :: derive (:: core :: clone :: Clone , :: core :: marker :: Copy)] # [allow (deprecated)] $ (# [$ attr]) * pub struct $ i { $ ($ field) * } }) ; }
};
}
