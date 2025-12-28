macro_rules! Imp {
    () => {
        # [cfg (not (feature = "alloc"))] # [derive (Clone , Debug)] struct Imp < 'a > (& 'a [u8]) ;
    };
}

Imp!();