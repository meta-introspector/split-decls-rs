macro_rules! HashStableContext {
    () => {
        # [doc = " Requirements for a `StableHashingContext` to be used in this crate."] # [doc = " This is a hack to allow using the `HashStable_Generic` derive macro"] # [doc = " instead of implementing everything in `rustc_middle`."] pub trait HashStableContext : rustc_span :: HashStableContext { }
    };
}

HashStableContext!();