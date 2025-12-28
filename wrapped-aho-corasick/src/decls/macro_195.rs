macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! macro_195 {
    () => {
        deps!();
        testconfig ! (search_teddy_ssse3_leftmost_first , PACKED_LEFTMOST_FIRST , | c : & mut Config | { c . only_teddy (true) ; # [cfg (target_arch = "x86_64")] if std :: is_x86_feature_detected ! ("ssse3") { c . only_teddy_256bit (Some (false)) ; } }) ;
    };
}

macro_195!()