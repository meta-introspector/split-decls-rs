macro_rules! deps {
    () => {
        ContextStack!();
    };
}

macro_rules! get_yield {
    () => {
        deps!();
        # [doc = " get the passed in para"] # [inline] # [deprecated (since = "0.6.18" , note = "please use `scope` version instead")] pub fn get_yield < A : Any > () -> Option < A > { let context = ContextStack :: current () . top () ; raw_get_yield (context) }
    };
}

get_yield!()