macro_rules! set_error_detail {
    () => {
        # [doc = " Sets whether information for more error details"] # [doc = " should be collected. This is useful for debugging"] # [doc = " parser errors (as it leads to more comprehensive"] # [doc = " error messages), but it has a higher performance cost."] # [doc = " (hence, it's off by default)"] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " * `enabled` - Whether to enable the collection for"] # [doc = "               more error details."] pub fn set_error_detail (enabled : bool) { ERROR_DETAIL . store (enabled , Ordering :: Relaxed) ; }
    };
}

set_error_detail!()