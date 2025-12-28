macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_516 {
    () => {
        deps!();
        impl < T , A : Allocator > Default for IntoIter < T , A > { # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { IntoIter { inner : Default :: default () , } } }
    };
}

impl_516!()