macro_rules! deps {
    () => {
        WasmSymbol!();
    };
}

macro_rules! impl_770 {
    () => {
        deps!();
        impl < 'data , 'file > read :: private :: Sealed for WasmSymbol < 'data , 'file > { }
    };
}

impl_770!()