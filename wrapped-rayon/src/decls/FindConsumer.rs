macro_rules! deps {
    () => {
        MatchPosition!();
    };
}

macro_rules! FindConsumer {
    () => {
        deps!();
        struct FindConsumer < 'p , P > { find_op : & 'p P , lower_bound : Cell < usize > , upper_bound : usize , match_position : MatchPosition , best_found : & 'p AtomicUsize , }
    };
}

FindConsumer!();