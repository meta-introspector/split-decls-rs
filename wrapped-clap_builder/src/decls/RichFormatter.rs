macro_rules! RichFormatter {
    () => {
        # [doc = " Richly formatted error context"] # [doc = ""] # [doc = " This follows the [rustc diagnostic style guide](https://rustc-dev-guide.rust-lang.org/diagnostics.html#suggestion-style-guide)."] # [non_exhaustive] # [cfg (feature = "error-context")] pub struct RichFormatter ;
    };
}

RichFormatter!();