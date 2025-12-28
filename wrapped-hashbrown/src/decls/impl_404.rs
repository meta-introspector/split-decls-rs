macro_rules! deps {
    () => {
        HashMap!();
        HashSet!();
    };
}

macro_rules! impl_404 {
    () => {
        deps!();
        impl < T , S , A > Default for HashSet < T , S , A > where S : Default , A : Default + Allocator , { # [doc = " Creates an empty `HashSet<T, S>` with the `Default` value for the hasher."] # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { Self { map : HashMap :: default () , } } }
    };
}

impl_404!()