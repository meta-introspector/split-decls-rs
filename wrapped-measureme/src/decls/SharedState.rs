macro_rules! deps {
    () => {
        BackingStorage!();
    };
}

macro_rules! SharedState {
    () => {
        deps!();
        # [doc = " This state is shared between all `SerializationSink`s writing to the same"] # [doc = " backing storage (e.g. the same file)."] # [derive (Clone , Debug)] struct SharedState (Arc < Mutex < BackingStorage > >) ;
    };
}

SharedState!()