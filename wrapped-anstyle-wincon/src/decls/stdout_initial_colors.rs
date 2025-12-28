macro_rules! deps {
    () => {
        StdioColorResult!();
        StdioColorInnerResult!();
    };
}

macro_rules! stdout_initial_colors {
    () => {
        deps!();
        # [doc = " Cached [`get_colors`] call for [`std::io::stdout`]"] pub fn stdout_initial_colors () -> StdioColorResult { static INITIAL : once_cell_polyfill :: sync :: OnceLock < StdioColorInnerResult > = once_cell_polyfill :: sync :: OnceLock :: new () ; (* INITIAL . get_or_init (| | get_colors_ (& std :: io :: stdout ()))) . map_err (Into :: into) }
    };
}

stdout_initial_colors!();