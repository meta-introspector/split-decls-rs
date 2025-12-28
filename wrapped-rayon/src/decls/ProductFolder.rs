macro_rules! ProductFolder {
    () => {
        struct ProductFolder < P > { product : P , }
    };
}

ProductFolder!()