macro_rules! deps {
    () => {
        Unstructured!();
    };
}

macro_rules! ArbitraryIter {
    () => {
        deps!();
        # [doc = " Utility iterator produced by [`Unstructured::arbitrary_iter`]"] pub struct ArbitraryIter < 'a , 'b , ElementType > { u : & 'b mut Unstructured < 'a > , _marker : PhantomData < ElementType > , }
    };
}

ArbitraryIter!()