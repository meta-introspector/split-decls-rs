macro_rules! deps {
    () => {
        SetLenOnDrop!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < 'a > SetLenOnDrop < 'a > { # [inline (always)] pub (super) fn new (len : & 'a mut usize) -> Self { SetLenOnDrop { local_len : * len , len , } } # [inline (always)] pub (super) fn increment_len (& mut self , increment : usize) { self . local_len += increment ; } }
    };
}

impl_146!();