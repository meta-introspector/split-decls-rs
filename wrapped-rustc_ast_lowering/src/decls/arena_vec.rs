macro_rules! arena_vec {
    () => {
        macro_rules ! arena_vec { ($ this : expr ; $ ($ x : expr) ,*) => ($ this . arena . alloc_from_iter ([$ ($ x) ,*])) ; }
    };
}

arena_vec!();