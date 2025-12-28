macro_rules! SharedOption {
    () => {
        type SharedOption < T > = Arc < Mutex < Option < T > > > ;
    };
}

SharedOption!()