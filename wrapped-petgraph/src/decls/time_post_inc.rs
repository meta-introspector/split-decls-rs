macro_rules! deps {
    () => {
        Time!();
    };
}

macro_rules! time_post_inc {
    () => {
        deps!();
        fn time_post_inc (x : & mut Time) -> Time { let v = * x ; x . 0 += 1 ; v }
    };
}

time_post_inc!()