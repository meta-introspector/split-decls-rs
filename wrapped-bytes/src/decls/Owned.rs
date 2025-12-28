macro_rules! Owned {
    () => {
        # [repr (C)] struct Owned < T > { ref_cnt : AtomicUsize , owner : T , }
    };
}

Owned!();