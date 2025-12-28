macro_rules! MakeBackendFn {
    () => {
        # [doc = " Function pointer type that constructs a new CodegenBackend."] type MakeBackendFn = fn () -> Box < dyn CodegenBackend > ;
    };
}

MakeBackendFn!();