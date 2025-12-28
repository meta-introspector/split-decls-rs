macro_rules! K {
    () => {
        # [cfg (target_pointer_width = "32")] const K : usize = 0x93d765dd ;
    };
}

K!();