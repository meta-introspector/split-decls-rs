macro_rules! deps {
    () => {
        LinkedHashSet!();
        LinkedHashMap!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < T , S > Default for LinkedHashSet < T , S > where S : Default , { # [inline] fn default () -> LinkedHashSet < T , S > { LinkedHashSet { map : LinkedHashMap :: default () , } } }
    };
}

impl_144!();