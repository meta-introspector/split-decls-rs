macro_rules! is_old_enough_to_be_collected {
    () => {
        fn is_old_enough_to_be_collected (timestamp : SystemTime) -> bool { timestamp < SystemTime :: now () - Duration :: from_secs (10) }
    };
}

is_old_enough_to_be_collected!();