macro_rules! deps {
    () => {
        NewTypeName!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl Default for NewTypeName { fn default () -> Self { Self :: Original } }
    };
}

impl_51!();