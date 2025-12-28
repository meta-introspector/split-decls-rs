macro_rules! deps {
    () => {
        WasmSymbolTable!();
    };
}

macro_rules! impl_764 {
    () => {
        deps!();
        impl < 'data , 'file > read :: private :: Sealed for WasmSymbolTable < 'data , 'file > { }
    };
}

impl_764!()