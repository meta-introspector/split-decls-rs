macro_rules! INCOMPATIBLE_FEATURES {
    () => {
        # [doc = " Some features are not allowed to be used together at the same time, if"] # [doc = " the two are present, produce an error."] pub const INCOMPATIBLE_FEATURES : & [(Symbol , Symbol)] = & [(sym :: ref_pat_eat_one_layer_2024 , sym :: ref_pat_eat_one_layer_2024_structural) ,] ;
    };
}

INCOMPATIBLE_FEATURES!()