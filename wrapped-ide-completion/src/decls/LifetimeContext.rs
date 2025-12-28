macro_rules! deps {
    () => {
        LifetimeKind!();
    };
}

macro_rules! LifetimeContext {
    () => {
        deps!();
        # [doc = " The state of the lifetime we are completing."] # [derive (Debug)] pub (crate) struct LifetimeContext { pub (crate) kind : LifetimeKind , }
    };
}

LifetimeContext!();