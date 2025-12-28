macro_rules! SharedVec {
    () => {
        type SharedVec < T > = Arc < Mutex < Vec < T > > > ;
    };
}

SharedVec!()