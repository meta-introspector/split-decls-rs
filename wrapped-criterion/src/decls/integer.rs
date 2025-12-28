macro_rules! integer {
    () => {
        # [doc = " Format a value as an integer, including thousands-separators."] pub fn integer (n : f64) -> String { thousands_sep (n as u64 , ',') }
    };
}

integer!();