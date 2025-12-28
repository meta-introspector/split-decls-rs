macro_rules! RawPtrToIntCast {
    () => {
        # [doc = " Casting raw pointer or function pointer to an integer."] # [doc = " Not currently intended to ever be allowed, even behind a feature gate: operation depends on"] # [doc = " allocation base addresses that are not known at compile-time."] # [derive (Debug)] pub (crate) struct RawPtrToIntCast ;
    };
}

RawPtrToIntCast!();