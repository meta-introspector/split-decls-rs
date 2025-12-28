macro_rules! Fuse {
    () => {
        # [doc = " Helper that sets a bool to `true` if dropped while unwinding."] # [derive (Clone)] struct Fuse < 'a > (& 'a AtomicBool) ;
    };
}

Fuse!();