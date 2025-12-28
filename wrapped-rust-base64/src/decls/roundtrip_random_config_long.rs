macro_rules! roundtrip_random_config_long {
    () => {
        # [test] fn roundtrip_random_config_long () { roundtrip_random_config (Uniform :: new (0 , 1000) , 10_000) ; }
    };
}

roundtrip_random_config_long!();