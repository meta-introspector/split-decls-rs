macro_rules! deps {
    () => {
        WasmFile!();
    };
}

macro_rules! impl_744 {
    () => {
        deps!();
        impl < 'data , R > read :: private :: Sealed for WasmFile < 'data , R > { }
    };
}

impl_744!()