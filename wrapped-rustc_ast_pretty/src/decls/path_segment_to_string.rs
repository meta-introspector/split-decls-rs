macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! path_segment_to_string {
    () => {
        deps!();
        pub fn path_segment_to_string (p : & ast :: PathSegment) -> String { State :: new () . path_segment_to_string (p) }
    };
}

path_segment_to_string!();