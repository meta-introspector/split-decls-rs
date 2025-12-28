macro_rules! deps {
    () => {
        StdioColorResult!();
    };
}

macro_rules! get_colors {
    () => {
        deps!();
        # [doc = " Get the colors currently active on the console"] pub fn get_colors < S : AsHandle > (stream : & S) -> StdioColorResult { get_colors_ (stream) . map_err (Into :: into) }
    };
}

get_colors!()