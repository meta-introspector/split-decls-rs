macro_rules! deps {
    () => {
        DefaultHasher!();
        DefaultHashBuilder!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl BuildHasher for DefaultHashBuilder { type Hasher = DefaultHasher ; # [inline] fn build_hasher (& self) -> Self :: Hasher { DefaultHasher (self . 0 . build_hasher ()) } }
    };
}

impl_6!()