macro_rules! deps {
    () => {
        HybridEngine!();
    };
}

macro_rules! Hybrid {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct Hybrid (Option < HybridEngine >) ;
    };
}

Hybrid!()