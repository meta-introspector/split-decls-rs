macro_rules! deps {
    () => {
        Builder!();
        CodegenCx!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < 'll , 'tcx > Deref for Builder < '_ , 'll , 'tcx > { type Target = CodegenCx < 'll , 'tcx > ; # [inline] fn deref (& self) -> & Self :: Target { self . cx } }
    };
}

impl_169!();