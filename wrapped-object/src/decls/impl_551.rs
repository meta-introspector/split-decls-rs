macro_rules! deps {
    () => {
        Bytes!();
        FunctionStartsIterator!();
    };
}

macro_rules! impl_551 {
    () => {
        deps!();
        impl < 'data > FunctionStartsIterator < 'data > { pub (super) fn new (data : & 'data [u8] , addr : u64) -> Self { FunctionStartsIterator { data : Bytes (data) , addr , } } }
    };
}

impl_551!()