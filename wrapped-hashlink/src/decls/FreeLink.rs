macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! FreeLink {
    () => {
        deps!();
        struct FreeLink < K , V > { next : Option < NonNull < Node < K , V > > > , }
    };
}

FreeLink!()