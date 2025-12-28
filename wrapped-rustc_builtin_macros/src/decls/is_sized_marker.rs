macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! is_sized_marker {
    () => {
        deps!();
        fn is_sized_marker (path : & ast :: Path) -> bool { const CORE_UNSIZE : [Symbol ; 3] = [sym :: core , sym :: marker , sym :: Sized] ; const STD_UNSIZE : [Symbol ; 3] = [sym :: std , sym :: marker , sym :: Sized] ; if path . segments . len () == 4 && path . is_global () { path_segment_is_exact_match (& path . segments [1 ..] , & CORE_UNSIZE) || path_segment_is_exact_match (& path . segments [1 ..] , & STD_UNSIZE) } else if path . segments . len () == 3 { path_segment_is_exact_match (& path . segments , & CORE_UNSIZE) || path_segment_is_exact_match (& path . segments , & STD_UNSIZE) } else { * path == sym :: Sized } }
    };
}

is_sized_marker!();