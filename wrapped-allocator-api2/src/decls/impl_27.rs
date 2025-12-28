macro_rules! deps {
    () => {
        Box!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        # [cfg (not (no_global_oom_handling))] impl < T : Default > Default for Box < T > { # [doc = " Creates a `Box<T>`, with the `Default` value for T."] # [inline (always)] fn default () -> Self { Box :: new (T :: default ()) } }
    };
}

impl_27!();