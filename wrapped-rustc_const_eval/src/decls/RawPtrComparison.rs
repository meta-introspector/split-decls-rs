macro_rules! RawPtrComparison {
    () => {
        # [doc = " Comparing raw pointers for equality."] # [doc = " Not currently intended to ever be allowed, even behind a feature gate: operation depends on"] # [doc = " allocation base addresses that are not known at compile-time."] # [derive (Debug)] pub (crate) struct RawPtrComparison ;
    };
}

RawPtrComparison!()