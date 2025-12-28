macro_rules! BoxFuture {
    () => {
        type BoxFuture < T > = Pin < Box < dyn Future < Output = T > + Send > > ;
    };
}

BoxFuture!()