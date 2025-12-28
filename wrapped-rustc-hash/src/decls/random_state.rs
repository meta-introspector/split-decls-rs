macro_rules! random_state {
    () => {
        # [cfg (feature = "rand")] mod random_state ;
    };
}

random_state!();