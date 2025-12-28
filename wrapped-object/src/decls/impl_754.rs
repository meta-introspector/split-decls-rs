macro_rules! deps {
    () => {
        WasmSection!();
    };
}

macro_rules! impl_754 {
    () => {
        deps!();
        impl < 'data , 'file , R > read :: private :: Sealed for WasmSection < 'data , 'file , R > { }
    };
}

impl_754!()