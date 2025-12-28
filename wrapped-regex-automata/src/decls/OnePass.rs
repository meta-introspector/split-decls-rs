macro_rules! deps {
    () => {
        OnePassEngine!();
    };
}

macro_rules! OnePass {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct OnePass (Option < OnePassEngine >) ;
    };
}

OnePass!();