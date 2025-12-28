macro_rules! deps {
    () => {
        Unstructured!();
    };
}

macro_rules! ArbitraryTakeRestIter {
    () => {
        deps!();
        # [doc = " Utility iterator produced by [`Unstructured::arbitrary_take_rest_iter`]"] pub struct ArbitraryTakeRestIter < 'a , ElementType > { u : Unstructured < 'a > , _marker : PhantomData < ElementType > , }
    };
}

ArbitraryTakeRestIter!();