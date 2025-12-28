macro_rules! deps {
    () => {
        ConstBuffer!();
        RuntimeType!();
        GUID!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl RuntimeType for GUID { const SIGNATURE : imp :: ConstBuffer = imp :: ConstBuffer :: from_slice (b"g16") ; }
    };
}

impl_133!();