macro_rules! perl_decimal {
    () => {
        # [cfg (all (feature = "unicode-perl" , not (feature = "unicode-gencat")))] # [allow (dead_code)] pub mod perl_decimal ;
    };
}

perl_decimal!();