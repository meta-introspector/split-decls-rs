macro_rules! assert_fused_stream {
    () => {
        # [doc (hidden)] # [inline (always)] pub fn assert_fused_stream < T : Stream + FusedStream > (_ : & T) { }
    };
}

assert_fused_stream!();