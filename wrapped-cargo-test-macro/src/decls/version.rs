macro_rules! version {
    () => {
        fn version () -> (u32 , bool) { LazyLock :: force (& VERSION) . clone () }
    };
}

version!();