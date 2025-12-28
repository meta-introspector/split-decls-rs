macro_rules! impl_1141 {
    () => {
        impl < R : AsyncBufRead > Lines < R > { pub (super) fn new (reader : R) -> Self { Self { reader , buf : String :: new () , bytes : Vec :: new () , read : 0 } } }
    };
}

impl_1141!()