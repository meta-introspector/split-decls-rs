macro_rules! native {
    () => {
        # [doc = " The arch triple of the test-running host."] pub fn native () -> & 'static str { env ! ("NATIVE_ARCH") }
    };
}

native!();