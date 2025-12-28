macro_rules! AtomicChoice {
    () => {
        # [derive (Debug)] pub (crate) struct AtomicChoice (AtomicUsize) ;
    };
}

AtomicChoice!();