macro_rules! other_1 {
    () => {
        # [allow (unused_macros)] macro do_not_use_safe_print ($ ($ t : tt) *) { std :: compile_error ! ("Don't use `safe_print` or `safe_println` here, use `println_info` instead") }
    };
}

other_1!()