macro_rules! deps {
    () => {
        BlameRanges!();
    };
}

macro_rules! Options {
    () => {
        deps!();
        # [doc = " Options to be passed to [`file()`](crate::file())."] # [derive (Default , Debug , Clone)] pub struct Options { # [doc = " The algorithm to use for diffing."] pub diff_algorithm : gix_diff :: blob :: Algorithm , # [doc = " The ranges to blame in the file."] pub ranges : BlameRanges , # [doc = " Don't consider commits before the given date."] pub since : Option < gix_date :: Time > , # [doc = " Determine if rename tracking should be performed, and how."] pub rewrites : Option < gix_diff :: Rewrites > , # [doc = " Collect debug information whenever there's a diff or rename that affects the outcome of a"] # [doc = " blame."] pub debug_track_path : bool , }
    };
}

Options!()