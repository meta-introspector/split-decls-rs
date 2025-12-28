macro_rules! deps {
    () => {
        Signature!();
        Type!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        impl Default for Signature { fn default () -> Self { Self { flags : MethodCallAttributes :: HASTHIS , return_type : Type :: Void , types : vec ! [] , } } }
    };
}

impl_274!();