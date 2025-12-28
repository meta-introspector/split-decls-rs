macro_rules! Float {
    () => {
        # [doc = " f64 parser from text"] struct Float < O , E > { o : PhantomData < O > , e : PhantomData < E > , }
    };
}

Float!()