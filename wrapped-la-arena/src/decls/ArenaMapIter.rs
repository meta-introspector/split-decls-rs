macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! ArenaMapIter {
    () => {
        deps!();
        pub struct ArenaMapIter < IDX , V > { iter : Enumerate < std :: vec :: IntoIter < Option < V > > > , _ty : PhantomData < IDX > , }
    };
}

ArenaMapIter!()