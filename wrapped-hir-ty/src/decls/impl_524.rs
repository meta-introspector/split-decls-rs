macro_rules! deps {
    () => {
        ExpressionStoreAdapter!();
    };
}

macro_rules! impl_524 {
    () => {
        deps!();
        impl < 'a , T > ExpressionStoreAdapter < 'a , T > { fn wrap (store : & 'a ExpressionStore) -> impl Fn (T) -> ExpressionStoreAdapter < 'a , T > { move | value | ExpressionStoreAdapter (value , store) } }
    };
}

impl_524!();