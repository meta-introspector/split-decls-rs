macro_rules! DropCounter {
    () => {
        # [doc = " This counter can create elements, and then count and verify"] # [doc = " the number of which have actually been dropped again."] # [derive (Default)] struct DropCounter { created : AtomicUsize , dropped : AtomicUsize , }
    };
}

DropCounter!();