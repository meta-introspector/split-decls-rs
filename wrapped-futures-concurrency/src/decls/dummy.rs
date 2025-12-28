macro_rules! dummy {
    () => {
        # [cfg (all (test , feature = "alloc"))] mod dummy ;
    };
}

dummy!()