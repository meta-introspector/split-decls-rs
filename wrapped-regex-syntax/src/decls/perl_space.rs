macro_rules! perl_space {
    () => {
        # [cfg (all (feature = "unicode-perl" , not (feature = "unicode-bool")))] # [allow (dead_code)] pub mod perl_space ;
    };
}

perl_space!()