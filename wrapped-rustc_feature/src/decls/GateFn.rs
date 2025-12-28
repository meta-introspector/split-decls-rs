macro_rules! deps {
    () => {
        Features!();
    };
}

macro_rules! GateFn {
    () => {
        deps!();
        type GateFn = fn (& Features) -> bool ;
    };
}

GateFn!();