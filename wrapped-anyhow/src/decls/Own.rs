macro_rules! Own {
    () => {
        # [repr (transparent)] pub struct Own < T > where T : ? Sized , { pub ptr : NonNull < T > , }
    };
}

Own!();