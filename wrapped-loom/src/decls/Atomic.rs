macro_rules! Atomic {
    () => {
        # [derive (Debug)] pub (crate) struct Atomic < T > { # [doc = " Atomic object"] state : rt :: Atomic < T > , }
    };
}

Atomic!()