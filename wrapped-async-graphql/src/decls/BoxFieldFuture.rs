macro_rules! deps {
    () => {
        ServerResult!();
    };
}

macro_rules! BoxFieldFuture {
    () => {
        deps!();
        type BoxFieldFuture < 'a > = Pin < Box < dyn Future < Output = ServerResult < (Name , Value) > > + 'a + Send > > ;
    };
}

BoxFieldFuture!()