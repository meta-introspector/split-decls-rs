macro_rules! deps {
    () => {
        Connection!();
        Batch!();
    };
}

macro_rules! impl_707 {
    () => {
        deps!();
        impl < 'conn , 'sql > Batch < 'conn , 'sql > { # [doc = " Constructor"] pub fn new (conn : & 'conn Connection , sql : & 'sql str) -> Self { Batch { conn , sql , tail : 0 } } }
    };
}

impl_707!()