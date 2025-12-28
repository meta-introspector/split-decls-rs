macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Context {
    () => {
        deps!();
        # [doc = " Parser implementation for [context]"] pub struct Context < F > { context : & 'static str , parser : F , }
    };
}

Context!();