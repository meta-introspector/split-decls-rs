macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl digest :: FixedOutputReset for Hasher { # [inline] fn finalize_into_reset (& mut self , out : & mut GenericArray < u8 , Self :: OutputSize >) { out . copy_from_slice (self . finalize () . as_bytes ()) ; self . reset () ; } }
    };
}

impl_130!();