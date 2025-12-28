macro_rules! ExpressionStoreAdapter {
    () => {
        struct ExpressionStoreAdapter < 'a , T > (T , & 'a ExpressionStore) ;
    };
}

ExpressionStoreAdapter!()