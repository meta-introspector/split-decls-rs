macro_rules! EmptyProducer {
    () => {
        # [doc = " Private empty producer"] struct EmptyProducer < T : Send > (PhantomData < T >) ;
    };
}

EmptyProducer!();