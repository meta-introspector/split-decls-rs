macro_rules! deps {
    () => {
        LinkedHashMap!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < K , V , S > Default for LinkedHashMap < K , V , S > where S : Default , { # [inline] fn default () -> Self { Self :: with_hasher (S :: default ()) } }
    };
}

impl_7!()