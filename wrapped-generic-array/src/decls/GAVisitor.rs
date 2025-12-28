macro_rules! GAVisitor {
    () => {
        struct GAVisitor < T , N > { _t : PhantomData < T > , _n : PhantomData < N > , }
    };
}

GAVisitor!();