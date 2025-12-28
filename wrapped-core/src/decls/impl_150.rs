macro_rules! deps {
    () => {
        RuntimeType!();
        IInspectable!();
        ConstBuffer!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl RuntimeType for IInspectable { const SIGNATURE : imp :: ConstBuffer = imp :: ConstBuffer :: from_slice (b"cinterface(IInspectable)") ; }
    };
}

impl_150!()