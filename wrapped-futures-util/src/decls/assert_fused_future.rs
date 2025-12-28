macro_rules! assert_fused_future {
    () => {
        # [doc (hidden)] # [inline (always)] pub fn assert_fused_future < T : Future + FusedFuture > (_ : & T) { }
    };
}

assert_fused_future!()