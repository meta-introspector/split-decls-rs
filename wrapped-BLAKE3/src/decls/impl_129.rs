macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl digest :: FixedOutput for Hasher { # [inline] fn finalize_into (self , out : & mut GenericArray < u8 , Self :: OutputSize >) { out . copy_from_slice (self . finalize () . as_bytes ()) ; } }
    };
}

impl_129!()