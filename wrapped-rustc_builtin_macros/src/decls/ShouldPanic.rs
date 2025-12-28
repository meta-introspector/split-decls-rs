macro_rules! ShouldPanic {
    () => {
        enum ShouldPanic { No , Yes (Option < Symbol >) , }
    };
}

ShouldPanic!();