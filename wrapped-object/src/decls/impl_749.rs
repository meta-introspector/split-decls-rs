macro_rules! deps {
    () => {
        WasmSegment!();
    };
}

macro_rules! impl_749 {
    () => {
        deps!();
        impl < 'data , 'file , R > read :: private :: Sealed for WasmSegment < 'data , 'file , R > { }
    };
}

impl_749!();