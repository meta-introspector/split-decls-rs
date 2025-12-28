macro_rules! other_0 {
    () => {
        # [allow (unused_macros)] macro do_not_use_print ($ ($ t : tt) *) { std :: compile_error ! ("Don't use `print` or `println` here, use `safe_print` or `safe_println` instead") }
    };
}

other_0!()