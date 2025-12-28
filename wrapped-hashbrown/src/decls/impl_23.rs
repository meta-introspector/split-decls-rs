macro_rules! deps {
    () => {
        DefaultHashBuilder!();
        DefaultHasher!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        # [cfg (feature = "default-hasher")] impl BuildHasher for DefaultHashBuilder { type Hasher = DefaultHasher ; # [inline (always)] fn build_hasher (& self) -> Self :: Hasher { DefaultHasher { inner : self . inner . build_hasher () , } } }
    };
}

impl_23!()