macro_rules! deps {
    () => {
        FieldsWith!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < 'a , F : FnMut (char) -> bool > FieldsWith < 'a , F > { fn new (bytes : & 'a [u8] , f : F) -> FieldsWith < 'a , F > { FieldsWith { f , bytes , chars : bytes . char_indices () } } }
    };
}

impl_86!();