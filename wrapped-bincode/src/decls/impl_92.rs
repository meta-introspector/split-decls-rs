macro_rules! deps {
    () => {
        IoWriter!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < 'a , W : std :: io :: Write > IoWriter < 'a , W > { pub fn new (writer : & 'a mut W) -> Self { Self { writer , bytes_written : 0 , } } pub const fn bytes_written (& self) -> usize { self . bytes_written } }
    };
}

impl_92!()