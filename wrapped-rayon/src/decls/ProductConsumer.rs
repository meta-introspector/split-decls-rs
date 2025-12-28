macro_rules! ProductConsumer {
    () => {
        struct ProductConsumer < P : Send > { _marker : PhantomData < * const P > , }
    };
}

ProductConsumer!()