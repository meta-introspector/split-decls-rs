macro_rules! Intrinsic {
    () => {
        # [derive (Debug , Copy , Clone)] pub (crate) struct Intrinsic { id : NonZero < c_uint > , }
    };
}

Intrinsic!()