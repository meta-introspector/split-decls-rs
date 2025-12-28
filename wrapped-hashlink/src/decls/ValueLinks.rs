macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! ValueLinks {
    () => {
        deps!();
        struct ValueLinks < K , V > { next : NonNull < Node < K , V > > , prev : NonNull < Node < K , V > > , }
    };
}

ValueLinks!();