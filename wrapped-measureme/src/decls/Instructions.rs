macro_rules! deps {
    () => {
        Counter!();
    };
}

macro_rules! Instructions {
    () => {
        deps!();
        # [doc = " \"Instructions retired\" hardware performance counter (userspace-only)."] # [doc = ""] # [doc = " Can be obtained with `Counter::by_name(\"instructions:u\")`."] pub struct Instructions { instructions : hw :: Counter , start : u64 , }
    };
}

Instructions!()