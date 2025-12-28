macro_rules! deps {
    () => {
        InferredIndex!();
    };
}

macro_rules! CurrentItem {
    () => {
        deps!();
        # [doc = " To build constraints, we visit one item (type, trait) at a time"] # [doc = " and look at its contents. So e.g., if we have"] # [doc = " ```ignore (illustrative)"] # [doc = " struct Foo<T> {"] # [doc = "     b: Bar<T>"] # [doc = " }"] # [doc = " ```"] # [doc = " then while we are visiting `Bar<T>`, the `CurrentItem` would have"] # [doc = " the `DefId` and the start of `Foo`'s inferreds."] struct CurrentItem { inferred_start : InferredIndex , }
    };
}

CurrentItem!();