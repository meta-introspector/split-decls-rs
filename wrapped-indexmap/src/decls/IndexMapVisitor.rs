macro_rules! IndexMapVisitor {
    () => {
        struct IndexMapVisitor < K , V , S > (PhantomData < (K , V , S) >) ;
    };
}

IndexMapVisitor!();