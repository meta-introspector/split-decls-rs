macro_rules! IndexSetVisitor {
    () => {
        struct IndexSetVisitor < T , S > (PhantomData < (T , S) >) ;
    };
}

IndexSetVisitor!();