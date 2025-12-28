macro_rules! path_segment_is_exact_match {
    () => {
        fn path_segment_is_exact_match (path_segments : & [ast :: PathSegment] , syms : & [Symbol]) -> bool { path_segments . iter () . zip (syms) . all (| (segment , & symbol) | segment . ident . name == symbol) }
    };
}

path_segment_is_exact_match!()