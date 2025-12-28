macro_rules! SumConsumer {
    () => {
        struct SumConsumer < S : Send > { _marker : PhantomData < * const S > , }
    };
}

SumConsumer!()