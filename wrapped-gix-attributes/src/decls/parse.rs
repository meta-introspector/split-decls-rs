macro_rules! parse {
    () => {
        # [doc = " Parse attribute assignments line by line from `bytes`, and fail the operation on error."] # [doc = ""] # [doc = " For leniency, ignore errors using `filter_map(Result::ok)` for example."] pub fn parse (bytes : & [u8]) -> parse :: Lines < '_ > { parse :: Lines :: new (bytes) }
    };
}

parse!()