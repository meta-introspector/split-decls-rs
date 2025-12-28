macro_rules! deps {
    () => {
        BuiltinDeriveFn!();
    };
}

macro_rules! BuiltinDerive {
    () => {
        deps!();
        pub (crate) struct BuiltinDerive (pub (crate) BuiltinDeriveFn) ;
    };
}

BuiltinDerive!()