macro_rules! deps {
    () => {
        Box!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        # [cfg (not (no_global_oom_handling))] impl Clone for Box < str > { # [inline (always)] fn clone (& self) -> Self { let buf : Box < [u8] > = self . as_bytes () . into () ; unsafe { Box :: from_raw (Box :: into_raw (buf) as * mut str) } } }
    };
}

impl_31!()