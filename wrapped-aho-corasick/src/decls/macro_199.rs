macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! macro_199 {
    () => {
        deps!();
        testconfig ! (search_teddy_fat_leftmost_first , PACKED_LEFTMOST_FIRST , | c : & mut Config | { c . only_teddy (true) ; # [cfg (target_arch = "x86_64")] if std :: is_x86_feature_detected ! ("avx2") { c . only_teddy_fat (Some (true)) ; } }) ;
    };
}

macro_199!();