macro_rules! deps {
    () => {
        PikeVM!();
    };
}

macro_rules! PikeVMEngine {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct PikeVMEngine (pikevm :: PikeVM) ;
    };
}

PikeVMEngine!()