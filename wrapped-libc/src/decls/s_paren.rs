macro_rules! s_paren {
    () => {
        # [doc = " Implement `Clone` and `Copy` for a tuple struct, as well as `Debug`, `Eq`, `Hash`,"] # [doc = " and `PartialEq` if the `extra_traits` feature is enabled."] # [doc = ""] # [doc = " This is the same as [`s`] but works for tuple structs."] macro_rules ! s_paren { ($ ($ (# [$ attr : meta]) * pub struct $ i : ident ($ ($ field : tt) *) ;) *) => ($ (__item ! { # [cfg_attr (feature = "extra_traits" , :: core :: prelude :: v1 :: derive (Debug , Eq , Hash , PartialEq))] # [:: core :: prelude :: v1 :: derive (:: core :: clone :: Clone , :: core :: marker :: Copy)] $ (# [$ attr]) * pub struct $ i ($ ($ field) *) ; }) *) ; }
    };
}

s_paren!()