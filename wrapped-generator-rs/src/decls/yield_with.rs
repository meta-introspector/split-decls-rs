macro_rules! deps {
    () => {
        ContextStack!();
    };
}

macro_rules! yield_with {
    () => {
        deps!();
        # [doc = " yield something without catch passed in para"] # [inline] # [deprecated (since = "0.6.18" , note = "please use `scope` version instead")] pub fn yield_with < T : Any > (v : T) { let env = ContextStack :: current () ; let context = env . top () ; raw_yield (& env , context , v) ; }
    };
}

yield_with!()