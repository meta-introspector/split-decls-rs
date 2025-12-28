macro_rules! deps {
    () => {
        Cache!();
    };
}

macro_rules! PikeVMCache {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub (crate) struct PikeVMCache (Option < pikevm :: Cache >) ;
    };
}

PikeVMCache!()