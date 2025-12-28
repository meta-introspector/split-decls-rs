macro_rules! deps {
    () => {
        ResolveState!();
    };
}

macro_rules! Inner {
    () => {
        deps!();
        struct Inner { start_time : DateTime < Utc > , end_time : DateTime < Utc > , resolves : Vec < ResolveState > , }
    };
}

Inner!();