macro_rules! outline {
    () => {
        # [doc = " This calls the passed function while ensuring it won't be inlined into the caller."] # [inline (never)] # [cold] fn outline < F : FnOnce () -> R , R > (f : F) -> R { f () }
    };
}

outline!();