macro_rules! deps {
    () => {
        StdioColorInnerResult!();
    };
}

macro_rules! get_colors_ {
    () => {
        deps!();
        fn get_colors_ < S : AsHandle > (stream : & S) -> StdioColorInnerResult { let handle = stream . as_handle () ; let handle = handle . as_raw_handle () ; let info = inner :: get_screen_buffer_info (handle) ? ; let (fg , bg) = inner :: get_colors (& info) ; Ok ((fg , bg)) }
    };
}

get_colors_!();