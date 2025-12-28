macro_rules! epoch {
    () => {
        # [cfg (all (feature = "alloc" , target_has_atomic = "ptr"))] mod epoch ;
    };
}

epoch!();