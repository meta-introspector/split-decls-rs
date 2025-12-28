macro_rules! deps {
    () => {
        ContextStack!();
    };
}

macro_rules! yield_now {
    () => {
        deps!();
        # [doc = " switch back to parent context"] # [inline] pub fn yield_now () { let env = ContextStack :: current () ; let cur = env . top () ; raw_yield_now (& env , cur) ; }
    };
}

yield_now!()