macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! Context {
    () => {
        deps!();
        # [doc = " Thread-local context used in select."] # [derive (Debug , Clone)] pub struct Context { inner : Arc < Inner > , }
    };
}

Context!();