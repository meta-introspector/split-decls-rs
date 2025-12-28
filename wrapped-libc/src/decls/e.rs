macro_rules! e {
    () => {
        # [doc = " Implement `Clone` and `Copy` for an enum, as well as `Debug`, `Eq`, `Hash`, and"] # [doc = " `PartialEq` if the `extra_traits` feature is enabled."] macro_rules ! e { ($ ($ (# [$ attr : meta]) * pub enum $ i : ident { $ ($ field : tt) * }) *) => ($ (__item ! { # [cfg_attr (feature = "extra_traits" , :: core :: prelude :: v1 :: derive (Debug , Eq , Hash , PartialEq))] # [:: core :: prelude :: v1 :: derive (:: core :: clone :: Clone , :: core :: marker :: Copy)] $ (# [$ attr]) * pub enum $ i { $ ($ field) * } }) *) ; }
    };
}

e!()