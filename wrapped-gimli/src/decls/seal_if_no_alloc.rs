macro_rules! seal_if_no_alloc {
    () => {
        # [cfg (not (feature = "read"))] pub (crate) mod seal_if_no_alloc { # [derive (Debug)] pub struct Sealed ; }
    };
}

seal_if_no_alloc!()