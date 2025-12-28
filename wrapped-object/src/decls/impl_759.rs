macro_rules! deps {
    () => {
        WasmComdat!();
    };
}

macro_rules! impl_759 {
    () => {
        deps!();
        impl < 'data , 'file , R > read :: private :: Sealed for WasmComdat < 'data , 'file , R > { }
    };
}

impl_759!()