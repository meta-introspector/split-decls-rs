macro_rules! Handover {
    () => {
        # [derive (Default)] # [repr (align (4))] struct Handover (AtomicUsize) ;
    };
}

Handover!();